#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;

mod day1;
mod io;

use clap::App;
use log::LevelFilter;
use simplelog::TermLogger;
use simplelog::CombinedLogger;
use simplelog::Config;

fn init_terminal_logger(log_level: LevelFilter) {
    CombinedLogger::init(vec![TermLogger::new(log_level, Config::default()).unwrap()]).unwrap();
    info!("Hello!");
    info!("Logger initialized with level {}", log_level);
}

fn main() {
    init_terminal_logger(LevelFilter::Debug);

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let mut has_arg = false;

    if matches.is_present("day1") ||matches.is_present("all") {
        day1::day1();
        has_arg = true;
    }

    if !has_arg {
        warn!("No argument given");
    }

    info!("Goodbye");
}
