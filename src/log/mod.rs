use extra::time::Tm;

pub mod nginx;


// HTTP log record
pub struct HTTPLogRecord<'r> {
    remote_addr: &'r str,
    local_time: Tm,
    host: &'r str,
    // Request time in milliseconds
    request_time: uint,
    method: &'r str,
    path: &'r str,
    status: u16,
    sent_bytes: uint,
    referer: &'r str,
    user_agent: &'r str,
}

pub trait LogProcessor {
    fn process(&mut self, record: HTTPLogRecord);
}
