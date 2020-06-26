use std::env;
use std::process::exit;

use getopts::Options;
use sysinfo::Pid;

#[derive(Debug)]
pub struct Config {
    pub pid_list: Vec<Pid>,
    pub interval: u64,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            pid_list: Vec::new(),
            interval: 1,
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

    opts.optopt("n", "interval", "Seconds to wait between updates", "SEC");
    opts.optflag("h", "help", "Display this help and exit");
    opts.optflag("v", "version", "Output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            print_usage(&program, &opts);
            exit(0);
        }
    };

    if matches.opt_present("version") {
        let name = env!("CARGO_PKG_NAME");
        let version = env!("CARGO_PKG_VERSION");
        println!("{} version {}", name, version);
        exit(0);
    }

    if matches.opt_present("help") || matches.free.is_empty() {
        print_usage(&program, &opts);
        exit(0);
    }

    if let Some(v) = matches.opt_str("interval") {
        if let Ok(n) = v.parse::<u64>() {
            conf.interval = if n < 1 { 1 } else { n };
        }
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
