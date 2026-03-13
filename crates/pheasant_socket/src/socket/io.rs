use pheasant_sys::*;

pub struct Recv<'a> {
    buffer: &'a mut Vec<u8>,
    options: u32,
}

pub struct Send<'a> {
    buffer: &'a Vec<u8>,
    options: u32,
}
