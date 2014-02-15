use std::fmt::Default;

use super::{LogStats, StatsItem, StatsMap};
use super::utils::{format_duration, format_bytes};


// TODO: Should be moved outside
static NUMBER_OF_ITEMS: uint = 8u;

pub struct LogStatsPrinter<'r> {
    priv stats: &'r LogStats,
}

impl<'r> LogStatsPrinter<'r> {
    pub fn new<'r>(stats: &'r LogStats) -> LogStatsPrinter<'r> {
        LogStatsPrinter{stats: stats}
    }

    pub fn print(&self) {
        print(&self.stats.clients, "by requests", "Clients");
        print(&self.stats.paths, "by requests", "Paths");
        print(&self.stats.methods, "by requests", "Methods");
        print(&self.stats.statuses, "by requests", "Statuses");
        print(&self.stats.referers, "by requests", "Referers");
        print(&self.stats.user_agents, "by requests", "User agents");
    }
}

fn print<T: IterBytes + Eq + Default>(mapping: &StatsMap<T>, title: &str,
        key_title: &str) {
    let mut items: ~[StatsItem<T>] = mapping.iter().collect();
    items.sort_by(|&(_, a), &(_, b)| b.requests.cmp(&a.requests));
    print_sorted(items, title, key_title);
}

fn print_sorted<T: IterBytes + Eq + Default>(sorted: &[StatsItem<T>],
        title: &str, key_title: &str) {
    println!("\n{} {}", key_title, title);
    println!("=====================================================\
              ============================");
    println!("{: <57} Requests Duration Bytes", key_title);
    println!("-----------------------------------------------------\
              ----------------------------");
    for &(client, stats) in sorted.iter().take(NUMBER_OF_ITEMS) {
        println!("{: <60.60} {: >5} {: >8} {: >5}",
                 *client,
                 stats.requests,
                 format_duration(stats.request_time),
                 format_bytes(stats.sent_bytes));
    }
}
