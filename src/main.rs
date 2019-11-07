extern crate clap;

pub mod config;
pub mod schema;
pub mod secret;

use config::Config;
use clap::{App, Arg, ArgMatches};

use std::fs::File;
use std::io::{self, BufReader};

fn main() {
    let args = args();
    let config = Config::new(args.value_of("config"));

    process(args.value_of("target").unwrap(), &config)
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

fn process(target: &str, config: &Config) {
    let out_file = File::create(config.file_name).unwrap();
    let in_file = File::open(target).unwrap();
    let reader = BufReader::new(in_file);

    for line in reader.lines() {
        process_line(&mut line)
    }
}

fn process_line(line: &mut str) -> &str {

}