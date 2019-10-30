use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

const DEFAULT: &'static str = "~/.config/filler/config.toml";

#[derive(Debug)]
pub struct Config {
    commands: Vec<HashMap<String, Command>>,
    placeholder: Placeholder,
    file_name: String
}

impl Config {
    fn new(filename: &str) -> Config {
        Config::from(filename) 
    }

    fn default() -> Config {
        match Path::new(DEFAULT).exists() {
            true => Config::from(DEFAULT),
            false => Config::env_only()
        }
    }

    fn from(filename: &str) -> Config {
        let file = File::open(filename);
    }

    fn env_only() -> Config {

    }
}

#[derive(Debug)]
pub struct Placeholder {
    separator: String,
    opening: String,
    closing: String
}

#[derive(Debug)]
pub struct Command {
    command: String,
    flags: Option<Vec<String>>,
    position: KeyPosition
}

#[derive(Debug)]
pub enum KeyPosition {
    First,
    Last
}