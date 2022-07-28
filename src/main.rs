use std::fmt;

use log::{info, LevelFilter};

/// spx:sys is the parent process (pid = 1) to all other neutron userland processes
fn main() {
    println!("Welcome to Neutron Userland!");

    // Maybe should add a timestamp?? Or initialise the logger first
    // Maybe just log the id of the next log file? And store the rest in the logfile itself
    // If possible, can also log to toml, or else have to parse the logfile
    let current_date = chrono::offset::Local::today();
    let logfile = format!("/sys/logs/spx-{}.log", current_date.to_string());

    simple_logging::log_to_file(logfile, LevelFilter::Info)
        .expect("Couldn't launch logger to /sys/logs/{}");

    // At /sys/logs/spx-<date>.log
    info!("Started sparx system");

    // Do spx:system things...
}

#[test]
fn test_curr_date() {
    let current_date = chrono::offset::Local::today();
    println!("current date = {current_date}");
}
