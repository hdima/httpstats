use std::str;
use std::hashmap::HashMap;

use log::{HTTPLogRecord, LogProcessor};


// TODO: Should be moved outside
static NUMBER_OF_ITEMS: uint = 8u;

struct ObjectStats {
    client: ~str,
    requests: uint,
    request_time: uint,
    sent_bytes: uint
}

pub struct LogStats {
    priv clients: HashMap<~str, ObjectStats>
}

impl LogStats {
    pub fn new() -> ~LogStats {
        ~LogStats{clients: HashMap::new()}
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

impl LogProcessor for LogStats {
    fn process(&mut self, record: HTTPLogRecord) {
        self.clients.insert_or_update_with(
            record.remote_addr.to_owned(),
            ObjectStats{
                client: record.remote_addr.to_owned(),
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
    let (mut n, mut pos, modifier) = if duration < 1000 {
            (duration, 3, ~"s")
        } else if duration < 60 * 1000 {
            (duration / 100, 1, ~"s")
        } else if duration < 60 * 60 * 1000 {
            (duration / 6000, 1, ~"m")
        } else {
            (duration / 360000, 1, ~"h")
        };
    while pos > 0 && n % 10 == 0 {
        n /= 10;
        pos -= 1;
    }
    let int_part = (n / pow(10, pos)).to_str();
    let mut fract_part = str::with_capacity(pos + 1);
    if pos != 0 && n != 0 {
        fract_part.push_char('.');
        while pos > 0 {
            let digit = (n / pow(10, pos - 1)) % 10;
            fract_part.push_char((digit as u8 + '0' as u8) as char);
            pos -= 1;
        }
    }
    int_part + fract_part + modifier
}

fn pow(n: uint, p: uint) -> uint {
    let mut num = 1;
    for _ in range(0, p) {
        num *= n;
    }
    num
}

fn format_bytes(mut bytes: uint) -> ~str {
    static modifiers: [&'static str, ..4] = ["G", "M", "K", ""];
    let mut i = modifiers.len() - 1;
    while bytes >= 1024 && i != 0 {
        bytes /= 1024;
        i -= 1;
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
        assert_eq!(~"0s", format_duration(0));
        assert_eq!(~"0.009s", format_duration(9));
        assert_eq!(~"0.09s", format_duration(90));
        assert_eq!(~"0.9s", format_duration(900));
        assert_eq!(~"0.999s", format_duration(999));
        assert_eq!(~"1s", format_duration(1000));
        assert_eq!(~"1s", format_duration(1009));
        assert_eq!(~"1.4s", format_duration(1400));
        assert_eq!(~"1.6s", format_duration(1600));
        assert_eq!(~"59s", format_duration(59000));
        assert_eq!(~"1m", format_duration(60000));
        assert_eq!(~"1.5m", format_duration(90000));
        assert_eq!(~"1h", format_duration(60 * 60 * 1000));
        assert_eq!(~"1.5h", format_duration(90 * 60 * 1000));
        assert_eq!(~"10h", format_duration(10 * 60 * 60 * 1000));
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
