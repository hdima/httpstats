use std::os;

use nginx::NginxLogParser;

mod nginx;

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("Usage: {} logfile", args[0]);
        os::set_exit_status(2);
    } else {
        let parser = NginxLogParser::new(args[1]);
        for line in parser {
            print!("Line: {}", line);
        }
    }
}
