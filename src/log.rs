use extra::time::Tm;


// HTTP log record
pub struct HTTPLogRecord {
    remote_addr: ~str,
    local_time: Tm,
    host: ~str,
    // Request time in milliseconds
    request_time: uint,
    method: ~str,
    path: ~str,
    status: u16,
    sent_bytes: uint,
    referer: ~str,
    user_agent: ~str,
}

pub trait LogProcessor {
    fn process(&mut self, record: HTTPLogRecord);
}
