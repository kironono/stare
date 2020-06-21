mod config;

use std::time;
use std::thread;
use sysinfo::{System, SystemExt};

fn main() {
    let config = config::parse_args();
    let mut threads = vec![];

    for &pid in config.pid_list.iter() {
        threads.push(
            thread::spawn(move || {
                let system = System::new_all();
                let sleep_duration = time::Duration::from_secs(1);
                loop {
                    match system.get_process(pid) {
                        None => {
                            break;
                        }
                        Some(_p) => {
                        }
                    }
                    thread::sleep(sleep_duration);
                }
            })
        );
    }

    for th in threads {
        th.join().unwrap();
    }
}
