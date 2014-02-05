use std::iter::Iterator;
use std::io::Buffer;

pub struct NginxLogParser<B> {
    priv buffer: B
}

impl<R: Buffer> NginxLogParser<R> {
    pub fn new(buffer: R) -> NginxLogParser<R> {
        NginxLogParser{buffer: buffer}
    }
}

impl<R: Buffer> Iterator<~str> for NginxLogParser<R> {
    fn next(&mut self) -> Option<~str> {
        self.buffer.read_line()
    }
}
