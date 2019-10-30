extern crate clap;

pub mod config;
pub mod schema;

use config::Config;
use clap::{App, Arg, ArgMatches};

fn main() {
    let args = args();
}

fn args() -> ArgMatches {
    App::new("filler")
         .version("1.0")
         .about("Fills in config files with sensitive data")
         .author("Glen Holcomb")
         .arg(Arg::with_name("config")
              .short("c")
              .long("config")
              .value_name("conf")
              .help("specify a config file")
              .takes_value(true))
         .get_matches()
}

fn config(args: ArgMatches) -> Config {
    match args.value_of("config") {
        Some(filename) => Config::new(filename),
        None => Config::default()
    }
}