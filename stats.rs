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
    println!("\n{}", message);
    println!("=====================================================\
              =============");
    println!("Client                                  \
              Request   Duration   Bytes");
    println!("-----------------------------------------------------\
              -------------");
    for stats in sorted.iter().take(NUMBER_OF_ITEMS) {
        println!("{: <40} {: >6} {: >10.3f} {: >7}",
                 stats.client, stats.requests,
                 (stats.request_time as f32) / 1000.0,
                 format_bytes(stats.sent_bytes));
    }
}

fn format_bytes(mut bytes: uint) -> ~str {
    static modifiers: [&'static str, ..5] = ["", "K", "M", "G", "T"];
    let mut i = 0;
    while bytes > 1024 || i == 4 {
        bytes /= 1024;
        i += 1;
    }
    bytes.to_str() + modifiers[i]
}
