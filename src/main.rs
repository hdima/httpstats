extern crate time;
extern crate libc;
extern crate getopts;
extern crate collections;

use std::os;
use std::io::util::ChainedReader;
use std::io::BufferedReader;
use getopts::{getopts, optopt, usage, OptGroup};

use stats::LogStats;
use stats::printer::LogStatsPrinter;
use log::nginx::NginxLogParser;
use gzreader::GzipReader;

mod log;
mod stats;
mod gzreader;


static DEFAULT_NUMBER_OF_ITEMS_TO_PRINT: uint = 10u;

fn parse(filenames: &[String], n: uint) {
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
    printer.print(n);
}

fn print_usage(msg: &str, program: &str, opts: &[OptGroup]) {
    println!("{}\n\nSimple HTTP statistics, version 0.0.2\n\n\
              Usage: {} LOGFILE...{}", msg, program, usage("", opts));
    os::set_exit_status(2);
}

fn items_to_print(opt: Option<String>) -> Result<uint, String> {
    match opt {
        Some(str_n) => {
            let n_opt: Option<uint> =  from_str(str_n.as_slice());
            match n_opt {
                Some(n) if n > 0 => Ok(n),
                _ => Err(str_n)
            }
        },
        None => Ok(DEFAULT_NUMBER_OF_ITEMS_TO_PRINT)
    }
}

fn main() {
    // TODO: We can use an application object here with smaller methods
    let args = os::args();
    let program = args[0].clone();
    let opts = [
        optopt("n", "", "number of items to print", "NUMBER")
    ];
    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => m,
        Err(err) => {
            print_usage(err.to_string().as_slice(), program.as_slice(), &opts);
            return;
        }
    };
    if !matches.free.is_empty() {
        match items_to_print(matches.opt_str("n")) {
            Ok(n) => parse(matches.free.as_slice(), n),
            Err(_str_n) => {
                print_usage("Invalid number of items provided",
                            program.as_slice(), &opts);
                return;
            }
        }
    } else {
        print_usage("No log files provided", program.as_slice(), &opts);
        return;
    }
}
