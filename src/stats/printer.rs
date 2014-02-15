use super::{LogStats, StatsKeyValue};
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
        let mut stats: ~[StatsKeyValue] = self.stats.clients.iter().collect();
        stats.sort_by(|&(_, a), &(_, b)| b.requests.cmp(&a.requests));
        print_sorted(stats, "By requests");
        stats.sort_by(|&(_, a), &(_, b)| b.request_time.cmp(&a.request_time));
        print_sorted(stats, "By request time");
        stats.sort_by(|&(_, a), &(_, b)| b.sent_bytes.cmp(&a.sent_bytes));
        print_sorted(stats, "By sent bytes");
    }
}

fn print_sorted(sorted: &[StatsKeyValue], message: &str) {
    println!("\n  {}", message);
    println!("=====================================================\
              =============");
    println!("Client                                 \
              Requests   Duration   Bytes");
    println!("-----------------------------------------------------\
              -------------");
    for &(client, stats) in sorted.iter().take(NUMBER_OF_ITEMS) {
        println!("{: <40} {: >6} {: >10} {: >7}",
                 *client, stats.requests,
                 format_duration(stats.request_time),
                 format_bytes(stats.sent_bytes));
    }
}
