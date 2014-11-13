use std::hash::Hash;

use std::collections::HashMap;
use std::collections::hash_map::{Occupied, Vacant};

use time::{Tm, Timespec};

use log::{HTTPLogRecord, LogProcessor, HTTPStatus};

pub mod printer;
mod utils;


struct ObjectStats {
    requests: u64,
    request_time: u64,
    sent_bytes: u64
}

type StatsItem<'r, T> = (&'r T, &'r ObjectStats);
type StatsMap<T> = HashMap<T, ObjectStats>;

pub struct LogStats {
    start: Option<Tm>,
    end: Option<Tm>,
    start_sec: Option<Timespec>,
    end_sec: Option<Timespec>,
    total: ObjectStats,
    clients: StatsMap<String>,
    hosts: StatsMap<String>,
    methods: StatsMap<String>,
    paths: StatsMap<String>,
    statuses: StatsMap<HTTPStatus>,
    referers: StatsMap<String>,
    user_agents: StatsMap<String>,
    hours: StatsMap<u8>,
    dates: StatsMap<String>,
    users: StatsMap<String>,
}

impl LogStats {
    pub fn new() -> LogStats {
        LogStats{
            start: None,
            end: None,
            start_sec: None,
            end_sec: None,
            total: ObjectStats{
                requests: 0,
                request_time: 0,
                sent_bytes: 0
                },
            clients: HashMap::with_capacity(100),
            hosts: HashMap::with_capacity(1),
            methods: HashMap::with_capacity(3), // GET, POST, HEAD
            paths: HashMap::with_capacity(20),
            statuses: HashMap::with_capacity(10),
            referers: HashMap::with_capacity(100),
            user_agents: HashMap::with_capacity(100),
            hours: HashMap::with_capacity(24),
            dates: HashMap::with_capacity(2),
            users: HashMap::with_capacity(1),
            }
    }
}

impl LogProcessor for LogStats {
    #[inline]
    fn process(&mut self, record: HTTPLogRecord) {
        update_interval(self, &record.local_time);
        update_totals(&mut self.total, &record);
        update(&mut self.clients, record.remote_addr.into_string(), &record);
        update(&mut self.hosts, record.host.into_string(), &record);
        update(&mut self.methods, record.method.into_string(), &record);
        update(&mut self.paths, record.path.into_string(), &record);
        update(&mut self.statuses, record.status, &record);
        update(&mut self.referers, record.referer.into_string(), &record);
        update(&mut self.user_agents, record.user_agent.into_string(), &record);
        update(&mut self.hours, record.local_time.tm_hour as u8, &record);
        update(&mut self.dates,
            record.local_time.strftime("%Y-%m-%d").unwrap().to_string(),
            &record);
        update(&mut self.users, record.user.into_string(), &record);
    }
}

#[inline]
fn update_interval(stats: &mut LogStats, current: &Tm) {
    let timespec = current.to_utc().to_timespec();
    match stats.start_sec {
        Some(start_sec) if start_sec <= timespec => {},
        _ => {
            stats.start_sec = Some(timespec);
            stats.start = Some(current.clone());
        }
    }
    match stats.end_sec {
        Some(end_sec) if end_sec >= timespec => {},
        _ => {
            stats.end_sec = Some(timespec);
            stats.end = Some(current.clone());
        }
    }
}

#[inline]
fn update_totals(totals: &mut ObjectStats,  record: &HTTPLogRecord) {
    totals.requests += 1;
    totals.request_time += record.request_time;
    totals.sent_bytes += record.sent_bytes;
}

#[inline]
fn update<T: Eq + Hash>(mapping: &mut StatsMap<T>, key: T,
        record: &HTTPLogRecord) {
    match mapping.entry(key) {
        Vacant(entry) => {
            entry.set(ObjectStats{requests: 1,
                                  request_time: record.request_time,
                                  sent_bytes: record.sent_bytes,
                                  });
        },
        Occupied(mut entry) => {
            let stats = entry.get_mut();
            stats.requests += 1;
            stats.request_time += record.request_time;
            stats.sent_bytes += record.sent_bytes;
        }
    };
}
