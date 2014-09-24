use time::{Tm, strptime};

use super::{HTTPLogRecord, LogProcessor, HTTPStatus};


pub struct NginxLogParser<B> {
    buffer: B
}

impl<B: Buffer> NginxLogParser<B> {
    pub fn new(buffer: B) -> NginxLogParser<B> {
        NginxLogParser{buffer: buffer}
    }

    pub fn parse<P: LogProcessor>(&mut self, processor: &mut P) {
        for result in self.buffer.lines() {
            let line = result.unwrap();
            let record = create_log_record(line.as_slice());
            processor.process(record);
        }
    }
}

#[inline]
fn create_log_record<'t>(line: &'t str) -> HTTPLogRecord<'t> {
    // TODO: Line parser can be implemented as iterator with a line
    // specification as input
    let (remote_addr, tail) = get_field(line);
    let (user, tail) = get_field(tail);
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
        user: user,
        request_time: request_time,
        method: method,
        path: path,
        status: HTTPStatus{status: status as u16},
        sent_bytes: sent_bytes,
        referer: referer,
        user_agent: user_agent,
        }
}

#[inline]
fn get_field<'t>(line: &'t str) -> (&'t str, &'t str) {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => (slice.slice_to(end), slice.slice_from(end + 1)),
        None => fail!("incomplete string: {}", line)
    }
}

#[inline]
fn get_field_or<'t>(line: &'t str, default: &'t str) -> (&'t str, &'t str) {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => (slice.slice_to(end), slice.slice_from(end + 1)),
        None => (default, "")
    }
}

#[inline]
fn skip_field<'t>(line: &'t str) -> &'t str {
    let slice = line.trim_left();
    match slice.find(' ') {
        Some(end) => slice.slice_from(end + 1),
        None => fail!("incomplete string: {}", line)
    }
}

#[inline]
fn get_delimited_field<'t>(line: &'t str, start_c: char, end_c: char) ->
        (&'t str, &'t str) {
    match line.trim_left().slice_shift_char() {
        (Some(c), slice) if c == start_c =>
            // FIXME: Should we skip escaped end characters? But probably
            // not so important in this case
            match slice.find(end_c) {
                Some(end) => (slice.slice_to(end), slice.slice_from(end + 1)),
                None => fail!("incomplete string: {}", line)
            },
        _ => fail!("incomplete string: {}", line)
    }
}

#[inline]
fn get_local_time<'t>(line: &'t str) -> (Tm, &'t str) {
    let (slice, tail) = get_delimited_field(line, '[', ']');
    match strptime(slice, "%d/%b/%Y:%H:%M:%S %z") {
        Ok(local_time) => (local_time, tail),
        Err(err) => fail!("time parse error for {}: {}", line, err)
    }
}

#[inline]
fn get_request_time<'t>(line: &'t str) -> (u64, &'t str) {
    let (slice, tail) = get_field(line);
    match slice.find('.') {
        Some(pos) => {
            let sec: u64 = from_str(slice.slice_to(pos)).unwrap();
            let msec: u64 = from_str(slice.slice_from(pos + 1)).unwrap();
            (sec * 1000 + msec, tail)
        }
        None => fail!("invalid request time: {}", line)
    }
}

#[inline]
fn get_method_path<'t>(line: &'t str) -> (&'t str, &'t str, &'t str) {
    let (slice, tail) = get_delimited_field(line, '"', '"');
    let (method, req_tail) = get_field_or(slice, "");
    let (path, _) = get_field_or(req_tail, "");
    (method, path, tail)
}

#[inline]
fn get_int<'t>(line: &'t str) -> (u64, &'t str) {
    let (slice, tail) = get_field(line);
    (from_str(slice).unwrap(), tail)
}
