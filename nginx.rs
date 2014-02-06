use std::iter::Iterator;
use std::io::Buffer;

use extra::time::{Tm, strptime};

// TODO: Probably should be moved to another module?
pub struct HTTPLogRecord {
    remote_addr: ~str,
    local_time: Tm,
    host: ~str,
}

pub struct NginxLogParser<B> {
    priv buffer: B
}

impl<R: Buffer> NginxLogParser<R> {
    pub fn new(buffer: R) -> NginxLogParser<R> {
        NginxLogParser{buffer: buffer}
    }
}

impl<R: Buffer> Iterator<HTTPLogRecord> for NginxLogParser<R> {
    fn next(&mut self) -> Option<HTTPLogRecord> {
        match self.buffer.read_line() {
            Some(line) => Some(create_log_record(line)),
            None => None
        }
    }
}

fn create_log_record(line: ~str) -> HTTPLogRecord {
    let (mut idx, remote_addr) = get_field(line, 0);
    // User
    idx = skip_field(line, idx);
    let (mut idx, local_time) = get_local_time(line, idx);
    // FIXME: We skip the trailing space with index increment here.
    // It's probably better to just skip spaces?
    let (_, host) = get_field(line, idx + 1);
    HTTPLogRecord{
        remote_addr: remote_addr,
        local_time: local_time,
        host: host,
        }
}

fn get_field(line: &str, start: uint) -> (uint, ~str) {
    let slice = line.slice_from(start);
    match slice.find(' ') {
        Some(end) => (start + end + 1, slice.slice_to(end).to_owned()),
        None => fail!("incomplete string: {}", line)
    }
}

fn skip_field(line: &str, start: uint) -> uint {
    let slice = line.slice_from(start);
    match slice.find(' ') {
        Some(end) => start + end + 1,
        None => fail!("incomplete string: {}", line)
    }
}

fn get_local_time(line: &str, start: uint) -> (uint, Tm) {
    let slice = line.slice_from(start);
    if slice.len() < 1 || slice[0] != '[' as u8 {
        fail!("incomplete string: {}", line);
    }
    match slice.find(']') {
        Some(end) => {
            let str_time = slice.slice(1, end);
            match strptime(str_time, "%d/%b/%Y:%H:%M:%S %z") {
                Ok(local_time) => (start + end + 1, local_time),
                Err(err) => fail!("time parse error for {}: {}", line, err)
            }
        }
        None => fail!("incomplete string: {}", line)
    }
}
