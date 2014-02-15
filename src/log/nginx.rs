use std::io::Buffer;

use extra::time::{Tm, strptime};

use super::{HTTPLogRecord, LogProcessor};


pub struct NginxLogParser<B> {
    priv buffer: B
}

impl<R: Buffer> NginxLogParser<R> {
    pub fn new(buffer: R) -> NginxLogParser<R> {
        NginxLogParser{buffer: buffer}
    }

    pub fn parse<P: LogProcessor>(&mut self, processor: &mut P) {
        for line in self.buffer.lines() {
            let record = create_log_record(line);
            processor.process(record);
        }
    }
}

#[inline]
fn create_log_record<'r>(line: &'r str) -> HTTPLogRecord<'r> {
    let (remote_addr, mut tail) = get_field(line);
    // User
    tail = skip_field(tail);
    let (local_time, tail) = get_local_time(tail);
    let (host, mut tail) = get_field(tail);
    // Pipe
    tail = skip_field(tail);
    let (request_time, tail) = get_request_time(tail);
    let (method, path, tail) = get_method_path(tail);
    let (status, tail) = get_int(tail);
    let (sent_bytes, tail) = get_int(tail);
    let (referer, tail) = get_delimited_field(tail, '"', '"');
    let (user_agent, _) = get_delimited_field(tail, '"', '"');
    HTTPLogRecord{
        remote_addr: remote_addr,
        local_time: local_time,
        host: host,
        request_time: request_time,
        method: method,
        path: path,
        status: status as u16,
        sent_bytes: sent_bytes as uint,
        referer: referer,
        user_agent: user_agent,
        }
}

#[inline]
fn get_field<'a>(line: &'a str) -> (&'a str, &'a str) {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => {
            (slice.slice_to(end), slice.slice_from(end + 1))
        }
        None => fail!("incomplete string: {}", line)
    }
}

#[inline]
fn get_field_or<'a>(line: &'a str, default: &'a str) -> (&'a str, &'a str) {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => {
            (slice.slice_to(end), slice.slice_from(end + 1))
        }
        None => (default, &'a "")
    }
}

#[inline]
fn skip_field<'a>(line: &'a str) -> &'a str {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => slice.slice_from(end + 1),
        None => fail!("incomplete string: {}", line)
    }
}

#[inline]
fn get_delimited_field<'a>(line: &'a str, start: char, end: char) ->
        (&'a str, &'a str) {
    let mut slice = line.trim_left();
    if slice.len() < 1 || slice[0] != start as u8 {
        fail!("incomplete string: {}", line);
    } else {
        slice = slice.slice_from(1);
        // FIXME: Should we skip escaped end characters? But probably not so
        // important in this case
        match slice.find(end) {
            Some(end) => {
                (slice.slice_to(end), slice.slice_from(end + 1))
            }
            None => fail!("incomplete string: {}", line)
        }
    }
}

#[inline]
fn get_local_time<'a>(line: &'a str) -> (Tm, &'a str) {
    let (slice, tail) = get_delimited_field(line, '[', ']');
    match strptime(slice, "%d/%b/%Y:%H:%M:%S %z") {
        Ok(local_time) => (local_time, tail),
        Err(err) => fail!("time parse error for {}: {}", line, err)
    }
}

#[inline]
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

#[inline]
fn get_method_path<'a>(line: &'a str) -> (&'a str, &'a str, &'a str) {
    let (slice, tail) = get_delimited_field(line, '"', '"');
    let (method, req_tail) = get_field_or(slice, "");
    let (path, _) = get_field_or(req_tail, "");
    (method, path, tail)
}

#[inline]
fn get_int<'a>(line: &'a str) -> (uint, &'a str) {
    let (slice, tail) = get_field(line);
    (from_str(slice).unwrap(), tail)
}
