extern crate clap;

pub mod config;
pub mod schema;
pub mod secret;

use config::Config;
use clap::{App, Arg, ArgMatches};

use std::fs::File;

fn main() {
    let args = args();
    let config = Config::new(args.value_of("config"));

    process(args.value_of("target").unwrap())
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
         .arg(Arg::with_name("target")
              .short("t")
              .long("target")
              .value_name("target")
              .help("the file to be processed")
              .required(true)
              .index(1))
         .arg(Arg::with_name("out")
              .short("o")
              .long("output")
              .value_name("out")
              .help("the output file to generate, if omitted the target file will be updated in place")
              .takes_value(true))
         .get_matches()
}

fn process(target: &str) {
    let mut file = File::open(target).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

}