use std::env;
use std::process::exit;

use getopts::{Options};
use sysinfo::Pid;

#[derive(Debug)]
pub struct Config {
    pub pid_list: Vec<Pid>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            pid_list: Vec::new()
        }
    }
}


fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} PID", program);
    print!("{}", opts.usage(&brief));
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut conf = Config::default();

    let mut opts = Options::new();

    opts.optflag("h", "help", "Show this help");
    opts.optflag("v", "version", "Print application version");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, &opts);
            exit(0);
        }
    };

    if matches.opt_present("version") {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}", version);
        exit(0);
    }

    if matches.opt_present("help") || matches.free.is_empty() {
        print_usage(&program, &opts);
        exit(0);
    }

    let pids = &matches.free[0];
    let is_pid = pids.chars().all(|c| c.is_numeric() || c == ',');
    if is_pid {
        for pd in pids.split(',') {
            if let Ok(i) = pd.parse::<usize>() {
                conf.pid_list.push(i as Pid);
            }
        }
    } else {
        eprintln!("Invalid Pids: {}", pids);
        exit(1);
    }

    conf
}
