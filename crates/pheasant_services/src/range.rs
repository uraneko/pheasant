use pheasant_http::{ErrorStatus, err_stt, server::Respond, status};
use std::io::{Read, Seek, SeekFrom, Write};

pub struct Ranges {
    ranges: Vec<[Option<usize>; 2]>,
    writable: bool,
}

/// asserts that this server resource supports the range header
pub fn support_ranges(headers: &mut Vec<u8>) {
    headers.extend(b"accept-ranges: bytes\n");
}

impl Ranges {
    pub fn new(value: &[u8], writable: bool) -> Result<Self, ErrorStatus> {
        if !value.starts_with(b"bytes=") {
            return err_stt!(?400);
        }

        let capa = value.iter().filter(|b| **b == b',').count() + 1;
        let mut ranges = Vec::with_capacity(if capa == 0 { 1 } else { capa });

        let mut zero = 6;
        while let Some(idx) = value[zero..].iter().position(|b| *b == b',') {
            ranges.push(parse_range(&value[zero..zero + idx])?);
            zero += idx + 1;
        }

        ranges.push(parse_range(&value[zero..])?);

        Ok(Self { ranges, writable })
    }

    pub fn meta(&self, resp: &mut Respond, len: usize, range_header: &[u8]) {
        resp.status(status!(206));
        // TODO fix this unwrap
        resp.headers_mut().extend(
            format!(
                "content-range: {}/{}\n",
                str::from_utf8(range_header).unwrap(),
                len
            )
            .as_bytes(),
        );
    }

    pub fn read(
        &self,
        seeker: &mut (impl Seek + Read),
        buf: &mut Vec<u8>,
    ) -> Result<usize, ErrorStatus> {
        let mut n = 0;
        for range in self.ranges.iter() {
            n += read_range(range, seeker, buf)?;
        }

        Ok(n)
    }

    pub fn write(
        &self,
        seeker: &mut (impl Seek + Write),
        buf: &[u8],
    ) -> Result<usize, ErrorStatus> {
        if !self.writable {
            return err_stt!(?422);
        }

        let mut n = 0;
        for range in self.ranges.iter() {
            n += write_range(range, seeker, buf)?;
        }

        Ok(n)
    }
}

fn read_start_end(
    start: usize,
    end: usize,
    seeker: &mut (impl Read + Seek),
    buf: &mut Vec<u8>,
) -> Result<usize, ErrorStatus> {
    let len = end - start + 1;
    buf.extend((0..len).into_iter().map(|_| 0));
    let blen = buf.len();
    seeker
        .seek(SeekFrom::Start(start as u64))
        .map_err(|_| err_stt!(416))?;
    seeker
        .read_exact(&mut buf[blen - len..])
        .map_err(|_| err_stt!(422))?;

    Ok(len)
}

fn read_start(
    start: usize,
    seeker: &mut (impl Read + Seek),
    buf: &mut Vec<u8>,
) -> Result<usize, ErrorStatus> {
    seeker
        .seek(SeekFrom::Start(start as u64))
        .map_err(|_| err_stt!(416))?;

    seeker.read_to_end(buf).map_err(|_| err_stt!(422))
}

fn read_end(
    end: usize,
    seeker: &mut (impl Read + Seek),
    buf: &mut Vec<u8>,
) -> Result<usize, ErrorStatus> {
    seeker
        .seek(SeekFrom::End(-(end as i64)))
        .map_err(|_| err_stt!(416))?;

    seeker.read_to_end(buf).map_err(|_| err_stt!(422))
}

// TODO redo this feeble implementation
pub fn read_range(
    range: &[Option<usize>; 2],
    seeker: &mut (impl Seek + Read),
    buf: &mut Vec<u8>,
) -> Result<usize, ErrorStatus> {
    Ok(match range {
        [Some(start), Some(end)] => read_start_end(*start, *end, seeker, buf)?,
        [Some(start), None] => read_start(*start, seeker, buf)?,
        [None, Some(end)] => read_end(*end, seeker, buf)?,
        [None, None] => return err_stt!(?416),
    })
}

fn write_start_end(
    start: usize,
    end: usize,
    seeker: &mut (impl Write + Seek),
    buf: &[u8],
) -> Result<usize, ErrorStatus> {
    let len = end - start + 1;
    if buf.len() != len {
        return err_stt!(?416);
    }

    seeker
        .seek(SeekFrom::Start(start as u64))
        .map_err(|_| err_stt!(416))?;
    seeker.write_all(&buf).map_err(|_| err_stt!(422))?;

    Ok(len)
}

fn write_start(
    start: usize,
    seeker: &mut (impl Write + Seek),
    buf: &[u8],
) -> Result<usize, ErrorStatus> {
    seeker
        .seek(SeekFrom::Start(start as u64))
        .map_err(|_| err_stt!(416))?;

    seeker.write_all(buf).map_err(|_| err_stt!(422))?;

    Ok(buf.len())
}

fn write_end(
    end: usize,
    seeker: &mut (impl Write + Seek),
    buf: &[u8],
) -> Result<usize, ErrorStatus> {
    seeker
        .seek(SeekFrom::End(-(end as i64)))
        .map_err(|_| err_stt!(416))?;

    seeker.write_all(buf).map_err(|_| err_stt!(422))?;

    Ok(buf.len())
}

pub fn write_range(
    range: &[Option<usize>; 2],
    seeker: &mut (impl Write + Seek),
    buf: &[u8],
) -> Result<usize, ErrorStatus> {
    Ok(match range {
        [Some(start), Some(end)] => write_start_end(*start, *end, seeker, buf)?,
        [Some(start), None] => write_start(*start, seeker, buf)?,
        [None, Some(end)] => write_end(*end, seeker, buf)?,
        [None, None] => return err_stt!(?416),
    })
}

// parses the range whatever its syntax
pub fn parse_range(range: &[u8]) -> Result<[Option<usize>; 2], ErrorStatus> {
    match range {
        r if r.starts_with(b"-") => parse_end(range),
        r if r.ends_with(b"-") => parse_start(range),
        r if r.contains(&b'-') => parse_start_end(range),
        _ => return err_stt!(?400),
    }
}

// the range is only and end half
// bytes=-432
pub fn parse_end(range: &[u8]) -> Result<[Option<usize>; 2], ErrorStatus> {
    if range[0] != b'-' {
        return err_stt!(?400);
    }

    let num = parse_num(&range[1..])?;

    Ok([None, Some(num)])
}

// the ramge is fully provided
// bytes=123-234
pub fn parse_start_end(range: &[u8]) -> Result<[Option<usize>; 2], ErrorStatus> {
    let Some(pos) = range.iter().position(|b| *b == b'-') else {
        return err_stt!(?400);
    };

    let start = parse_num(&range[..pos])?;
    let end = parse_num(&range[pos + 1..])?;

    Ok([Some(start), Some(end)])
}

// the range is only a start half
// bytes=234-
pub fn parse_start(range: &[u8]) -> Result<[Option<usize>; 2], ErrorStatus> {
    let len = range.len();
    if range[len - 1] != b'-' {
        return err_stt!(?400);
    }
    let num = parse_num(&range[..len - 1])?;

    Ok([Some(num), None])
}

pub fn parse_num(num: &[u8]) -> Result<usize, ErrorStatus> {
    let Ok(s) = str::from_utf8(num) else {
        return err_stt!(?400);
    };

    s.parse::<usize>().map_err(|_| err_stt!(400))
}
