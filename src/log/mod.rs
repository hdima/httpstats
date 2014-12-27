use std::fmt::{Show, Formatter, Result};
use std::hash::Hash;

use time::Tm;

use self::utils::http_status_description;

pub mod nginx;
mod utils;


#[deriving(PartialEq, Eq, Hash, Copy)]
pub struct HTTPStatus {
    status: u16,
}

impl Show for HTTPStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let desc = http_status_description(self.status);
        let msg = self.status.to_string() + " " + desc;
        f.pad(msg.as_slice())
    }
}

// HTTP log record
pub struct HTTPLogRecord<'r> {
    pub remote_addr: &'r str,
    pub local_time: Tm,
    pub host: &'r str,
    pub user: &'r str,
    // Request time in milliseconds
    pub request_time: u64,
    pub method: &'r str,
    pub path: &'r str,
    pub status: HTTPStatus,
    pub sent_bytes: u64,
    pub referer: &'r str,
    pub user_agent: &'r str,
}

pub trait LogProcessor {
    fn process(&mut self, record: HTTPLogRecord);
}
