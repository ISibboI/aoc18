#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.subcommand_name().is_none() {
        panic!("No subcommand given");
    }

    match matches.subcommand_name().unwrap() {
        c => panic!("Unknown subcommand: {}", c)
    }
}
