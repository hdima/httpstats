use std::hashmap::HashMap;

use log::{HTTPLogRecord, LogProcessor};

pub mod printer;
mod utils;


struct ObjectStats {
    requests: uint,
    request_time: uint,
    sent_bytes: uint
}

type StatsItem<'r, T> = (&'r T, &'r ObjectStats);
type StatsMap<T> = HashMap<T, ObjectStats>;

pub struct LogStats {
    priv clients: StatsMap<~str>,
    priv methods: StatsMap<~str>,
    priv paths: StatsMap<~str>,
    priv statuses: StatsMap<u16>,
    priv referers: StatsMap<~str>,
    priv user_agents: StatsMap<~str>,
}

impl LogStats {
    pub fn new() -> ~LogStats {
        ~LogStats{
            clients: HashMap::new(),
            methods: HashMap::new(),
            paths: HashMap::new(),
            statuses: HashMap::new(),
            referers: HashMap::new(),
            user_agents: HashMap::new(),
            }
    }
}

impl LogProcessor for LogStats {
    fn process(&mut self, record: HTTPLogRecord) {
        update(&mut self.clients, record.remote_addr.into_owned(), &record);
        update(&mut self.methods, record.method.into_owned(), &record);
        update(&mut self.paths, record.path.into_owned(), &record);
        update(&mut self.statuses, record.status, &record);
        update(&mut self.referers, record.referer.into_owned(), &record);
        update(&mut self.user_agents, record.user_agent.into_owned(), &record);
    }
}

fn update<T: IterBytes + Eq>(mapping: &mut StatsMap<T>, key: T,
        record: &HTTPLogRecord) {
    mapping.insert_or_update_with(
        key,
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