use std::fmt::{Default, Formatter};

use extra::time::Tm;

use self::utils::http_status_description;

pub mod nginx;
mod utils;


#[deriving(Eq)]
#[deriving(IterBytes)]
pub struct HTTPStatus {
    status: u16,
}

impl Default for HTTPStatus {
    fn fmt(status: &HTTPStatus, f: &mut Formatter) {
        let desc = http_status_description(status.status);
        f.pad(status.status.to_str() + " " + desc);
    }
}

// HTTP log record
pub struct HTTPLogRecord<'r> {
    remote_addr: &'r str,
    local_time: Tm,
    host: &'r str,
    // Request time in milliseconds
    request_time: uint,
    method: &'r str,
    path: &'r str,
    status: HTTPStatus,
    sent_bytes: uint,
    referer: &'r str,
    user_agent: &'r str,
}

pub trait LogProcessor {
    fn process(&mut self, record: HTTPLogRecord);
}
