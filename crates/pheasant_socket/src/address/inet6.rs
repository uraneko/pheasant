// WIP this module is a WIP
// still unusable
use crate::*;

pub struct In6Addr {
    addr: [u8; 16],
}

// ::1 is loopback (localhost)
// ::0 is any = ipv4 0.0.0.0
impl In6Addr {
    pub fn to_bytes(&self) -> [u8; 16] {
        self.addr
    }

    pub fn new(addr: [u8; 16]) -> Self {
        Self { addr }
    }

    // 01:23:45:67:89:ff:ab:cd
    // the last 4 bytes ab:cd are the ipv4
    pub fn v4_mapped(o0: u8, o1: u8, o2: u8, o3: u8) -> Self {
        Self::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, o0, o1, o2, o3])
    }

    // only valid loopback is ::1
    pub fn loopback() -> Self {
        Self::new([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
    }

    // ::
    pub fn any() -> Self {
        Self::new([0; _])
    }
}

impl core::fmt::Debug for In6Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}:{:X}{:X}",
            self.addr[0],
            self.addr[1],
            self.addr[2],
            self.addr[3],
            self.addr[4],
            self.addr[5],
            self.addr[6],
            self.addr[7],
            self.addr[8],
            self.addr[9],
            self.addr[10],
            self.addr[11],
            self.addr[12],
            self.addr[13],
            self.addr[14],
            self.addr[15],
        )
    }
}

fn parse_addr(addr: &str) -> Result<In6Addr, ConversionError> {
    if addr.chars().filter(|ch| *ch == ':').count() > 15 {
        return Err(ConversionError::InvalidStr);
    }
    let tokens = parse::Lex::new(addr.as_bytes()).lex()?;

    todo!()
}

pub enum ConversionError {
    InvalidStr,
    ExpectedDoubleColon,
    TooShort,
}

impl core::str::FromStr for In6Addr {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "::" => Ok(Self::any()),
            "::1" => Ok(Self::loopback()),
            addr => parse_addr(addr),
        }
    }
}

// 28 bytes
#[repr(C)]
pub struct SockAddrIn6 {
    // AF_INET6
    family: u16,
    port: u16,
    // keep at 0
    flowinfo: u32,
    addr: In6Addr,
    // keep at 0
    scope_id: u32,
}

impl SockAddrIn6 {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
    pub const AF: AddressFamily = AddressFamily::Inet6;

    pub fn new(addr: impl Into<In6Addr>, port: u16) -> Self {
        let port = u16::from_be(port);
        Self {
            family: Self::AF.into(),
            addr: addr.into(),
            port,
            flowinfo: 0,
            scope_id: 0,
        }
    }
}

pub mod parse {
    use super::*;
    extern crate alloc;
    use alloc::vec::Vec;

    pub enum Token<'a> {
        Colon,
        // 1 at the end of ::1
        One,
        // 0 from abcd:0:1234
        // which is equivalent to abcd:0000:1234
        Zero,
        DoubleColon,
        // 16bits abcd or def4 in abcd::def4::
        Word16(&'a [u8]),
    }

    pub struct Lex<'a> {
        buffer: &'a [u8],
        cursor: usize,
    }

    impl<'a> Lex<'a> {
        pub fn new(buffer: &'a [u8]) -> Self {
            Self { cursor: 0, buffer }
        }

        pub fn lex(&mut self) -> Result<Vec<Token>, ConversionError> {
            let mut tokens = Vec::new();
            while self.cursor < self.buffer.len() - 1 {
                self.lex_once(&mut tokens)?;
            }

            Ok(tokens)
        }

        pub fn lex_once(&mut self, tokens: &mut Vec<Token<'a>>) -> Result<(), ConversionError> {
            if self.buffer[self.cursor] == b':' {
                self.lex_colon(tokens)
            } else {
                self.lex_segment(tokens)
            }
        }

        pub fn lex_colon(&mut self, tokens: &mut Vec<Token>) -> Result<(), ConversionError> {
            if self.cursor == 0 {
                if self.buffer[1] != b':' {
                    return Err(ConversionError::ExpectedDoubleColon);
                }
                self.cursor += 2;
                tokens.push(Token::DoubleColon);
            } else {
                if self.buffer[self.cursor + 1] == b':' {
                    self.cursor += 2;
                    tokens.push(Token::DoubleColon);
                } else {
                    self.cursor += 1;
                    tokens.push(Token::Colon);
                }
            }

            Ok(())
        }

        pub fn lex_segment(&mut self, tokens: &mut Vec<Token<'a>>) -> Result<(), ConversionError> {
            // should be eql if segment is the last
            // or greater if segment is not the last
            if self.buffer.len() < self.cursor + 4 {
                return Err(ConversionError::TooShort);
            }
            let segment = &self.buffer[self.cursor..self.cursor + 4];
            self.cursor += 4;
            if self.buffer.len() > self.cursor && self.buffer[self.cursor] != b':' {
                return Err(ConversionError::InvalidStr);
            }
            tokens.push(Token::Word16(segment));

            Ok(())
        }
    }
}

// ::abcd     -> 0000:0000:0000:abcd
// abcd::1234 -> abcd:0000:0000:1234
// abcd::     -> abcd:0000:0000:0000
//
