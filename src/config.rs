extern crate rusoto_core;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::from_utf8;
use std::path::Path;
use std::process;

const DEFAULT: &'static str = "~/.config/filler/config.json";

#[derive(Debug, Deserialize)]
pub struct Config {
    commands: HashMap<String, Command>,
    placeholder: Placeholder,
    file_name: String
}

impl Config {
    pub fn new(filename: Option<&str>) -> Config {
        match filename {
            Some(filename) => Config::from(filename),
            None => Config::default()
        }
    }

    fn from(filename: &str) -> Config {
        let mut file = File::open(filename).unwrap();
        let mut buffer = Vec::<u8>::new();

        file.read(buffer.as_mut_slice());

        let contents = from_utf8(buffer.as_mut_slice()).unwrap();

        serde_json::from_str(contents).unwrap()
    }

    pub fn command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }
}

impl Default for Config {
    fn default() -> Self {
        match Path::new(DEFAULT).exists() {
            true => Config::from(DEFAULT),
            false => {
                println!("No config file specified and none found at: {}", DEFAULT);
                process::exit(1);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Placeholder {
    separator: String,
    opening: String,
    closing: String
}

#[derive(Debug, Deserialize)]
pub struct Command {
    name: String,
    command: String,
    flags: Option<Vec<String>>,
    position: KeyPosition
}

impl Command {
    pub fn run(&self, key: &str) -> Option<String> {
        let no_args = Vec::<String>::new();
        let flags = match &self.flags {
            Some(f) => f,
            None => &no_args
        };

        let command = match self.position {
                          KeyPosition::First => {
                              process::Command::new(&self.command)
                                               .arg(key)
                                               .args(flags)
                                               .output()
                          },
                          KeyPosition::Last => {
                              process::Command::new(&self.command)
                                               .args(flags)
                                               .arg(key)
                                               .output()
            }
        };

        match command {
            Ok(result) => {
                match String::from_utf8(result.stdout) {
                    Ok(out) => Some(out),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum KeyPosition {
    First,
    Last
}