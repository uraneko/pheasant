#![no_std]
use pheasant_sys::*;

pub mod socket;

pub struct TcpSocket {
    fd: u32,
    addr: u32,
    port: u16,
}

impl TcpSocket {
    pub fn new() -> Self {
        todo!()
    }
}
