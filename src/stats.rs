use std::fmt;
use std::hashmap::HashMap;

use log::HTTPLogRecord;


static NUMBER_OF_ITEMS: uint = 8u;

struct ObjectStats {
    client: ~str,
    requests: uint,
    request_time: uint,
    sent_bytes: uint
}

pub struct Stats {
    priv clients: HashMap<~str, ObjectStats>
}

impl Stats {
    pub fn new() -> Stats {
        Stats{clients: HashMap::new()}
    }

    pub fn update(&mut self, record: HTTPLogRecord) {
        match self.clients.find_mut(&record.remote_addr) {
            Some(stats) => {
                stats.requests += 1;
                stats.request_time += record.request_time;
                stats.sent_bytes += record.sent_bytes;
                return;
            }
            None => ()
        }
        let stats = ObjectStats{
            client: record.remote_addr.clone(),
            requests: 1,
            request_time: record.request_time,
            sent_bytes: record.sent_bytes,
            };
        self.clients.insert(record.remote_addr, stats);
    }

    pub fn print(&self) {
        let mut sorted: ~[&ObjectStats] = self.clients.values().collect();
        sorted.sort_by(|a, b| b.requests.cmp(&a.requests));
        print_sorted(sorted, "By requests");
        sorted.sort_by(|a, b| b.request_time.cmp(&a.request_time));
        print_sorted(sorted, "By request time");
        sorted.sort_by(|a, b| b.sent_bytes.cmp(&a.sent_bytes));
        print_sorted(sorted, "By sent bytes");
    }
}

fn print_sorted(sorted: &[&ObjectStats], message: &str) {
    println!("\n  {}", message);
    println!("=====================================================\
              =============");
    println!("Client                                 \
              Requests   Duration   Bytes");
    println!("-----------------------------------------------------\
              -------------");
    for stats in sorted.iter().take(NUMBER_OF_ITEMS) {
        println!("{: <40} {: >6} {: >10} {: >7}",
                 stats.client, stats.requests,
                 format_duration(stats.request_time),
                 format_bytes(stats.sent_bytes));
    }
}

fn format_duration(duration: uint) -> ~str {
    // TODO: Cleanup code
    if duration == 0 {
        ~"0"
    } else if duration < 1000 {
        if duration % 100 == 0 {
            format_args!(fmt::format, "0.{}s", duration / 100)
        } else if duration % 10 == 0 {
            format_args!(fmt::format, "0.{:02u}s", duration / 10)
        } else {
            format_args!(fmt::format, "0.{:03u}s", duration)
        }
    } else if duration < 60 * 1000 {
        format_args!(fmt::format, "{}s{}", duration / 1000, duration % 1000)
    } else if duration < 60 * 60 * 1000 {
        format_args!(fmt::format, "{}m{}s", duration / (60 * 1000),
                     (duration / 1000) % 60)
    } else {
        format_args!(fmt::format, "{}h{}m{}s", duration / (60 * 60 * 1000),
                     (duration / (60 * 1000)) % 60, duration % (60 * 1000))
    }
}

fn format_bytes(mut bytes: uint) -> ~str {
    static modifiers: [&'static str, ..4] = ["", "K", "M", "G"];
    let mut i = 0;
    let max = modifiers.len() - 1;
    while bytes >= 1024 && i != max {
        bytes /= 1024;
        i += 1;
    }
    bytes.to_str() + modifiers[i]
}

/*
 * Tests
 */
#[cfg(test)]
mod test {
    use super::{format_duration, format_bytes};

    #[test]
    fn test_format_duration() {
        assert_eq!(~"0", format_duration(0));
        assert_eq!(~"0.009s", format_duration(9));
        assert_eq!(~"0.09s", format_duration(90));
        assert_eq!(~"0.9s", format_duration(900));
        assert_eq!(~"0.999s", format_duration(999));
        // TODO
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(~"0", format_bytes(0));
        assert_eq!(~"9", format_bytes(9));
        assert_eq!(~"90", format_bytes(90));
        assert_eq!(~"900", format_bytes(900));
        assert_eq!(~"1000", format_bytes(1000));
        assert_eq!(~"1K", format_bytes(1024));
        assert_eq!(~"2K", format_bytes(2048));
        assert_eq!(~"4K", format_bytes(4096));
        assert_eq!(~"1M", format_bytes(1024 * 1024));
        assert_eq!(~"1G", format_bytes(1024 * 1024 * 1024));
        assert_eq!(~"1024G", format_bytes(1024 * 1024 * 1024 * 1024));
    }
}
