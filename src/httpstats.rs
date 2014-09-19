extern crate time;
extern crate libc;
extern crate collections;

use std::os;
use std::io::util::ChainedReader;
use std::io::BufferedReader;

use stats::LogStats;
use stats::printer::LogStatsPrinter;
use log::nginx::NginxLogParser;
use gzreader::GzipReader;

mod log;
mod stats;
mod gzreader;


static NUMBER_OF_ITEMS_TO_PRINT: uint = 10u;

fn parse(filenames: &[String]) {
    let files = filenames.iter().map(|filename| {
        let path = Path::new(filename.clone());
        GzipReader::open(&path).unwrap()
        });
    let file = ChainedReader::new(files);
    let reader = BufferedReader::new(file);
    let mut stats = LogStats::new();
    let mut parser = NginxLogParser::new(reader);
    parser.parse(&mut stats);
    let printer = LogStatsPrinter::new(&stats);
    printer.print(NUMBER_OF_ITEMS_TO_PRINT);
}

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("Simple HTTP statistics, version 0.0.2\n\n\
                  Usage: {} LOGFILE...", args[0]);
        os::set_exit_status(2);
    } else {
        parse(args.slice_from(1));
    }
}
