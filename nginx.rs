use std::iter::Iterator;
use std::io::Buffer;
use std::io::fs::File;
use std::io::buffered::BufferedReader;

pub struct NginxLogParser<'r> {
    priv buffer: &'r mut Buffer
}

impl<'r> NginxLogParser<'r> {
    pub fn new(filename: &str) -> ~NginxLogParser {
        let path = Path::new(filename);
        match File::open(&path) {
            Some(file) => {
                let mut reader = BufferedReader::new(file);
                ~NginxLogParser{buffer: &mut reader}
            }
            // FIXME: It seems doesn't work
            None => fail!("Unable to open {}", filename)
        }
    }
}

impl<'r> Iterator<~str> for NginxLogParser<'r> {
    fn next(&mut self) -> Option<~str> {
        self.buffer.read_line()
    }
}
