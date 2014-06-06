use std::hash::Hash;

use collections::hashmap::HashMap;

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
    priv start: Option<Tm>,
    priv end: Option<Tm>,
    priv start_sec: Option<Timespec>,
    priv end_sec: Option<Timespec>,
    priv total: ObjectStats,
    priv clients: StatsMap<~str>,
    priv hosts: StatsMap<~str>,
    priv methods: StatsMap<~str>,
    priv paths: StatsMap<~str>,
    priv statuses: StatsMap<HTTPStatus>,
    priv referers: StatsMap<~str>,
    priv user_agents: StatsMap<~str>,
    priv hours: StatsMap<u8>,
    priv dates: StatsMap<~str>,
    priv users: StatsMap<~str>,
}

impl LogStats {
    pub fn new() -> ~LogStats {
        ~LogStats{
            start: None,
            end: None,
            start_sec: None,
            end_sec: None,
            total: ObjectStats{
                requests: 0,
                request_time: 0,
                sent_bytes: 0
                },
            clients: HashMap::new(),
            hosts: HashMap::new(),
            methods: HashMap::new(),
            paths: HashMap::new(),
            statuses: HashMap::new(),
            referers: HashMap::new(),
            user_agents: HashMap::new(),
            hours: HashMap::new(),
            dates: HashMap::new(),
            users: HashMap::new(),
            }
    }
}

impl LogProcessor for LogStats {
    #[inline]
    fn process(&mut self, record: HTTPLogRecord) {
        update_interval(self, &record.local_time);
        update_totals(&mut self.total, &record);
        update(&mut self.clients, record.remote_addr.into_owned(), &record);
        update(&mut self.hosts, record.host.into_owned(), &record);
        update(&mut self.methods, record.method.into_owned(), &record);
        update(&mut self.paths, record.path.into_owned(), &record);
        update(&mut self.statuses, record.status, &record);
        update(&mut self.referers, record.referer.into_owned(), &record);
        update(&mut self.user_agents, record.user_agent.into_owned(), &record);
        update(&mut self.hours, record.local_time.tm_hour as u8, &record);
        update(&mut self.dates, record.local_time.strftime("%Y-%m-%d"),
            &record);
        update(&mut self.users, record.user.into_owned(), &record);
    }
}

#[inline]
fn update_interval(stats: &mut LogStats, current: &Tm) {
    let timespec = current.to_utc().to_timespec();
    match stats.start_sec {
        None => {
            stats.start_sec = Some(timespec);
            stats.start = Some(current.clone());
        }
        Some(start_sec) => {
            if start_sec > timespec {
                stats.start_sec = Some(timespec);
                stats.start = Some(current.clone());
            }
        }
    }
    match stats.end_sec {
        None => {
            stats.end_sec = Some(timespec);
            stats.end = Some(current.clone());
        }
        Some(end_sec) => {
            if end_sec < timespec {
                stats.end_sec = Some(timespec);
                stats.end = Some(current.clone());
            }
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
fn update<T: TotalEq + Hash>(mapping: &mut StatsMap<T>, key: T,
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
