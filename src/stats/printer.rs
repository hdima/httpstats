use std::fmt::Show;
use std::hash::Hash;

use time::Tm;

use super::{LogStats, StatsItem, StatsMap, ObjectStats};
use super::utils::{format_duration, format_bytes, format_number};


pub struct LogStatsPrinter<'r> {
    stats: &'r LogStats,
}

impl<'r> LogStatsPrinter<'r> {
    pub fn new<'r>(stats: &'r LogStats) -> LogStatsPrinter<'r> {
        LogStatsPrinter{stats: stats}
    }

    pub fn print(&self, limit: uint) {
        print_totals(&self.stats.total, &self.stats.start, &self.stats.end);
        print(&self.stats.hosts, "Requests", "Hosts", limit);
        print(&self.stats.dates, "Requests", "Dates", limit);
        print(&self.stats.users, "Requests", "Users", limit);
        print(&self.stats.clients, "Requests", "Clients", limit);
        print(&self.stats.hours, "Requests", "Hours", limit);
        print(&self.stats.paths, "Requests", "Paths", limit);
        print(&self.stats.methods, "Requests", "Methods", limit);
        print(&self.stats.statuses, "Requests", "Statuses", limit);
        print(&self.stats.referers, "Requests", "Referers", limit);
        print(&self.stats.user_agents, "Requests", "User agents", limit);
    }
}

#[inline]
fn print_totals(totals: &ObjectStats, start: &Option<Tm>, end: &Option<Tm>) {
    let start_date = match *start {
        None => "-".to_string(),
        Some(ref s) => s.strftime("%Y-%m-%d")
    };
    let end_date = match *end {
        None => "-".to_string(),
        Some(ref e) => e.strftime("%Y-%m-%d")
    };
    println!("Totals\n\
              =====================================================\
              ============================\n\
              Period                                                    \
              Requests Duration Bytes\n\
              -----------------------------------------------------\
              ----------------------------");
    println!("{: <10} - {: >10}                                   \
              {: >8} {: >8} {: >5}",
             start_date,
             end_date,
             format_number(totals.requests),
             format_duration(totals.request_time),
             format_bytes(totals.sent_bytes));
}

#[inline]
fn print<T: Eq + Hash + Show>(mapping: &StatsMap<T>, title: &str,
        key_title: &str, limit: uint) {
    let mut items: Vec<StatsItem<T>> = mapping.iter().collect();
    items.sort_by(|&(_, a), &(_, b)| b.requests.cmp(&a.requests));
    print_sorted(items, title, key_title, limit);
}

#[inline]
fn print_sorted<T: Show>(sorted: Vec<StatsItem<T>>,
        title: &str, key_title: &str, limit: uint) {
    println!("\n{} by {} (top {})\n\
              =====================================================\
              ============================\n\
              {: <57} Requests Duration Bytes\n\
              -----------------------------------------------------\
              ----------------------------",
              title, key_title, limit, key_title);
    for &(client, stats) in sorted.iter().take(limit) {
        println!("{: <60.60} {: >5} {: >8} {: >5}",
                 *client,
                 format_number(stats.requests),
                 format_duration(stats.request_time),
                 format_bytes(stats.sent_bytes));
    }
}
