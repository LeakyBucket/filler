extern crate clap;
extern crate regex;

pub mod config;
pub mod schema;
pub mod secret;

use schema::Context;

use config::{Config, Placeholder};
use clap::{App, Arg, ArgMatches};
use regex::Captures;

use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::process;

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
    let mut out_file = File::create(Path::new(&config.file_name)).unwrap();
    let in_file = File::open(Path::new(target)).unwrap();
    let reader = BufReader::new(in_file);
    let regex = config.placeholder.regex();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let new_line = regex.replace_all(&line, |caps: &Captures| {
                    let source = caps.name("source").unwrap().as_str();
                    let label = caps.name("label").unwrap().as_str();
                    let version = match caps.name("version") {
                        Some(cap) => Some(cap.as_str()),
                        None => None
                    };
                    let context = Context::new(&config.placeholder, source, label, version);

                    context.evaluate(config)
                });

                out_file.write(new_line.as_bytes());
            },
            Err(_) => {
                println!("Error reading {}", target);
                process::exit(1);
            }
        }
    }
}