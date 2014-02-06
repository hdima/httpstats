extern mod extra;

use std::os;
use std::io::fs::File;
use std::io::buffered::BufferedReader;

use nginx::NginxLogParser;

mod nginx;

fn parse(filename: &str) {
    let path = Path::new(filename);
    let file = File::open(&path).unwrap();
    let reader = BufferedReader::new(file);
    let mut parser = NginxLogParser::new(reader);
    for record in parser {
        println!("{:?}", record);
    }
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("Usage: {} logfile", args[0]);
        os::set_exit_status(2);
    } else {
        parse(args[1]);
    }
}
