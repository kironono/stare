mod config;

use std::thread;
use std::time;
use sysinfo::{System, SystemExt};

fn main() {
    let config = config::parse_args();
    let check_interval = time::Duration::from_secs(config.interval);
    let mut threads = vec![];

    for &pid in config.pid_list.iter() {
        threads.push(thread::spawn(move || {
            let mut system = System::new_all();
            loop {
                system.refresh_processes();
                match system.get_process(pid) {
                    None => {
                        break;
                    }
                    Some(_p) => {}
                }
                thread::sleep(check_interval);
            }
        }));
    }

    for th in threads {
        th.join().unwrap();
    }
}
