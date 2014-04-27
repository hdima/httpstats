use std::fmt::{Show, Formatter, Result};
use std::hash::Hash;

use time::Tm;

use self::utils::http_status_description;

pub mod nginx;
mod utils;


#[deriving(Eq, Hash, TotalEq)]
pub struct HTTPStatus {
    status: u16,
}

impl Show for HTTPStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let desc = http_status_description(self.status);
        f.pad(self.status.to_str() + " " + desc)
    }
}

// HTTP log record
pub struct HTTPLogRecord<'r> {
    remote_addr: &'r str,
    local_time: Tm,
    host: &'r str,
    // Request time in milliseconds
    request_time: u64,
    method: &'r str,
    path: &'r str,
    status: HTTPStatus,
    sent_bytes: u64,
    referer: &'r str,
    user_agent: &'r str,
}

pub trait LogProcessor {
    fn process(&mut self, record: HTTPLogRecord);
}
