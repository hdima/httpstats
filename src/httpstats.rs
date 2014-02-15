extern mod extra;

use std::os;
use std::io::fs::File;
use std::io::util::ChainedReader;
use std::io::buffered::BufferedReader;

use stats::LogStats;
use log::nginx::NginxLogParser;

mod log;
mod stats;


fn parse(filenames: &[~str]) {
    let files = filenames.iter().map(|filename| {
        let path = Path::new(filename.clone());
        File::open(&path).unwrap()
        });
    let file = ChainedReader::new(files);
    let reader = BufferedReader::new(file);
    let mut stats = LogStats::new();
    let mut parser = NginxLogParser::new(reader);
    parser.parse(stats);
    stats.print();
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Usage: {} LOGFILE...", args[0]);
        os::set_exit_status(2);
    } else {
        parse(args.slice_from(1));
    }
}
