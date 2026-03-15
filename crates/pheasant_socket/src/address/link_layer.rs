// this module is TODO

pub struct SockAddrLl {
    family: u16,
    // e.g., tcp, udp, ip?
    proto: u16,
    // e.g., wlan0, enp0s3
    ifindex: i32,
    hatype: u16,
    pkttype: u8,
    halen: u8,
    addr: [u8; 8],
}

impl SockAddrLl {
    pub const SIZE: u32 = core::mem::size_of::<Self>() as u32;
    pub const AF: AddressFamily = AddressFamily::Packet;
}
