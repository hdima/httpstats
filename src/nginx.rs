use std::iter::Iterator;
use std::io::Buffer;

use extra::time::{Tm, strptime};

use log::HTTPLogRecord;


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
    let (remote_addr, mut tail) = get_field(line);
    // User
    tail = skip_field(tail);
    let (local_time, tail) = get_local_time(tail);
    let (host, mut tail) = get_field(tail);
    // Pipe
    tail = skip_field(tail);
    let (request_time, tail) = get_request_time(tail);
    let (method, path, tail) = get_method_path(tail);
    let (status, tail) = get_status(tail);
    let (sent_bytes, tail) = get_sent_bytes(tail);
    // FIXME: What to do with "-" for referer and user agent? Should we
    // replace them with empty string or None for example?
    let (referer, tail) = get_delimited_field(tail, '"', '"');
    let (user_agent, _) = get_delimited_field(tail, '"', '"');
    HTTPLogRecord{
        remote_addr: remote_addr,
        local_time: local_time,
        host: host,
        request_time: request_time,
        method: method,
        path: path,
        status: status,
        sent_bytes: sent_bytes,
        referer: referer,
        user_agent: user_agent,
        }
}

fn get_field<'a>(line: &'a str) -> (~str, &'a str) {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => {
            (slice.slice_to(end).into_owned(), slice.slice_from(end + 1))
        }
        None => fail!("incomplete string: {}", line)
    }
}

fn get_field_or<'a>(line: &'a str, default: ~str) -> (~str, &'a str) {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => {
            (slice.slice_to(end).into_owned(), slice.slice_from(end + 1))
        }
        None => (default, &'a "")
    }
}

fn skip_field<'a>(line: &'a str) -> &'a str {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => slice.slice_from(end + 1),
        None => fail!("incomplete string: {}", line)
    }
}

fn get_delimited_field<'a>(line: &'a str, start: char, end: char) ->
        (~str, &'a str) {
    let mut slice = line.trim_left();
    if slice.len() < 1 || slice[0] != start as u8 {
        fail!("incomplete string: {}", line);
    } else {
        slice = slice.slice_from(1);
        // FIXME: Should we skip escaped end characters? But probably not so
        // important in this case
        match slice.find(end) {
            Some(end) => {
                (slice.slice_to(end).into_owned(), slice.slice_from(end + 1))
            }
            None => fail!("incomplete string: {}", line)
        }
    }
}

fn get_local_time<'a>(line: &'a str) -> (Tm, &'a str) {
    let (slice, tail) = get_delimited_field(line, '[', ']');
    match strptime(slice, "%d/%b/%Y:%H:%M:%S %z") {
        Ok(local_time) => (local_time, tail),
        Err(err) => fail!("time parse error for {}: {}", line, err)
    }
}

fn get_request_time<'a>(line: &'a str) -> (uint, &'a str) {
    let (slice, tail) = get_field(line);
    match slice.find('.') {
        Some(pos) => {
            let sec: int = from_str(slice.slice_to(pos)).unwrap();
            let msec: int = from_str(slice.slice_from(pos + 1)).unwrap();
            ((sec * 1000 + msec) as uint, tail)
        }
        None => fail!("invalid request time: {}", line)
    }
}

fn get_method_path<'a>(line: &'a str) -> (~str, ~str, &'a str) {
    let (slice, tail) = get_delimited_field(line, '"', '"');
    let (method, req_tail) = get_field_or(slice, ~"");
    let (path, _) = get_field_or(req_tail, ~"");
    (method, path, tail)
}

fn get_status<'a>(line: &'a str) -> (u16, &'a str) {
    let (slice, tail) = get_field(line);
    let status: int = from_str(slice).unwrap();
    (status as u16, tail)
}

fn get_sent_bytes<'a>(line: &'a str) -> (uint, &'a str) {
    let (slice, tail) = get_field(line);
    let bytes: int = from_str(slice).unwrap();
    (bytes as uint, tail)
}
