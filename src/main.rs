extern crate clap;
extern crate regex;

pub mod config;
pub mod schema;
pub mod secret;

use schema::Context;

use clap::{App, Arg, ArgMatches};
use config::Config;
use regex::Captures;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

fn main() {
    let args = args();
    let config = Config::new(args.value_of("config"));

    process(&args, &config)
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
              .value_name("target")
              .help("the file to be processed")
              .required(true)
              .index(1))
         .arg(Arg::with_name("out")
              .short("o")
              .long("output")
              .value_name("out")
              .help("the output file to generate, if omitted the target file name will have .filled appended to it")
              .takes_value(true))
         .get_matches()
}

fn process(args: &ArgMatches, config: &Config) {
    let target = args.value_of("target").unwrap();
    let output = match args.value_of("out") {
        Some(out) => out.to_owned(),
        None => format!("{}.filled", args.value_of("target").unwrap()),
    };
    let mut out_file = File::create(Path::new(&output)).unwrap();
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
                        None => None,
                    };
                    let context = Context::new(&config.placeholder, source, label, version);

                    context.evaluate(config)
                });

                out_file
                    .write(new_line.as_bytes())
                    .expect("Unable to write to output file!");
                out_file
                    .write(b"\n")
                    .expect("Unable to write to output file!");
            }
            Err(_) => {
                println!("Error reading {}", target);
                process::exit(1);
            }
        }
    }
}
