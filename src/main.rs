extern crate clap;

pub mod config;
pub mod schema;
pub mod secret;

use config::Config;
use clap::{App, Arg, ArgMatches};

fn main() {
    let args = args();
    let config = Config::new(args.value_of("config"));

    process(args.value_of("target"))
}

fn args() -> ArgMatches<'static> {
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

fn process(target: String) {

}