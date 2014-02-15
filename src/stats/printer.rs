use std::fmt::Default;

use super::{LogStats, StatsItem, StatsMap};
use super::utils::{format_duration, format_bytes};


pub struct LogStatsPrinter<'r> {
    priv stats: &'r LogStats,
}

impl<'r> LogStatsPrinter<'r> {
    pub fn new<'r>(stats: &'r LogStats) -> LogStatsPrinter<'r> {
        LogStatsPrinter{stats: stats}
    }

    pub fn print(&self, limit: uint) {
        print(&self.stats.clients, "by requests", "Clients", limit);
        print(&self.stats.paths, "by requests", "Paths", limit);
        print(&self.stats.methods, "by requests", "Methods", limit);
        print(&self.stats.statuses, "by requests", "Statuses", limit);
        print(&self.stats.referers, "by requests", "Referers", limit);
        print(&self.stats.user_agents, "by requests", "User agents", limit);
    }
}

fn print<T: IterBytes + Eq + Default>(mapping: &StatsMap<T>, title: &str,
        key_title: &str, limit: uint) {
    let mut items: ~[StatsItem<T>] = mapping.iter().collect();
    items.sort_by(|&(_, a), &(_, b)| b.requests.cmp(&a.requests));
    print_sorted(items, title, key_title, limit);
}

fn print_sorted<T: IterBytes + Eq + Default>(sorted: &[StatsItem<T>],
        title: &str, key_title: &str, limit: uint) {
    println!("\n{} {}", key_title, title);
    println!("=====================================================\
              ============================");
    println!("{: <57} Requests Duration Bytes", key_title);
    println!("-----------------------------------------------------\
              ----------------------------");
    for &(client, stats) in sorted.iter().take(limit) {
        println!("{: <60.60} {: >5} {: >8} {: >5}",
                 *client,
                 stats.requests,
                 format_duration(stats.request_time),
                 format_bytes(stats.sent_bytes));
    }
}
