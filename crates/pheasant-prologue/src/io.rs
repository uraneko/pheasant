pub trait ReadHeaders: Read {
    fn read_header(header: &[u8], buf: &mut [u8]) -> Option<&Header> {}
}
