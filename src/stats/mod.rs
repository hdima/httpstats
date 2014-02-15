use std::hashmap::HashMap;

use log::{HTTPLogRecord, LogProcessor};

pub mod printer;
mod utils;


struct ObjectStats {
    requests: uint,
    request_time: uint,
    sent_bytes: uint
}

pub type StatsKeyValue<'r> = (&'r ~str, &'r ObjectStats);

pub struct LogStats {
    priv clients: HashMap<~str, ObjectStats>,
}

impl LogStats {
    pub fn new() -> ~LogStats {
        ~LogStats{clients: HashMap::new()}
    }
}

impl LogProcessor for LogStats {
    fn process(&mut self, record: HTTPLogRecord) {
        self.clients.insert_or_update_with(
            record.remote_addr.into_owned(),
            ObjectStats{
                requests: 1,
                request_time: record.request_time,
                sent_bytes: record.sent_bytes,
                },
            |_, stats| {
                stats.requests += 1;
                stats.request_time += record.request_time;
                stats.sent_bytes += record.sent_bytes;
            });
    }
}
