extern crate dirs;
extern crate regex;
extern crate rusoto_core;

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;

const DEFAULT: &'static str = "/.config/filler/config.json";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub commands: HashMap<String, Command>,
    #[serde(default)]
    pub placeholder: Placeholder,
}

impl Config {
    pub fn new(filename: Option<&str>) -> Config {
        match filename {
            Some(filename) => Config::from(filename),
            None => Config::default(),
        }
    }

    fn from(filename: &str) -> Config {
        let path = Path::new(filename);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        serde_json::from_str(contents.as_str()).unwrap()
    }

    pub fn command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }
}

impl Default for Config {
    fn default() -> Self {
        let home = match dirs::home_dir() {
            Some(place) => place,
            None => {
                println!("Cannot determine home directory (for default config)");
                process::exit(1);
            }
        };
        let default_config = format!("{}{}", home.as_path().display(), DEFAULT);
        let path = Path::new(&default_config);

        match Path::new(path).exists() {
            true => Config::from(&default_config),
            false => {
                println!(
                    "No config file specified and none found at: {}{}",
                    home.as_path().display(),
                    DEFAULT
                );
                process::exit(1);
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Placeholder {
    #[serde(default)]
    pub separator: String,
    #[serde(default)]
    pub opening: String,
    #[serde(default)]
    pub closing: String,
}

impl Placeholder {
    pub fn regex(&self) -> Regex {
        match Regex::new(format!("{}", self).as_str()) {
            Ok(regex) => regex,
            Err(_) => {
                println!(
                    "Unable to build regex for placeholder pattern: {}, {}, {}",
                    self.opening, self.separator, self.closing
                );
                process::exit(1);
            }
        }
    }
}

impl fmt::Display for Placeholder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let escapes = Regex::new(r"(?P<c>[\{\[\}\]\?\.\*\+])").unwrap();
        let opening = escapes.replace_all(self.opening.as_str(), r"\$c");
        let separator = escapes.replace_all(self.separator.as_str(), r"\$c");
        let closing = escapes.replace_all(self.closing.as_str(), r"\$c");

        write!(f, "({}\\s*(?P<source>[^{}]+){}(?P<label>[^{}^\\s]+)({}(?P<version>[^{}^\\s]+)\\s*{}|\\s*{}))", opening, separator, separator, separator, separator, separator, closing, closing)
    }
}

impl Default for Placeholder {
    fn default() -> Self {
        Placeholder {
            opening: "{{".to_string(),
            separator: ":".to_string(),
            closing: "}}".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Command {
    command: String,
    flags: Option<Vec<String>>,
    version: Option<VersionArg>,
    position: KeyPosition,
}

impl Command {
    pub fn new(name: &str, flags: Option<Vec<String>>, ver: Option<VersionArg>, pos: KeyPosition) -> Self {
        Command {
            command: name.to_owned(),
            flags,
            version: ver,
            position: pos
        }
    }

    pub fn run(&self, key: &str, ver: Option<&str>) -> Option<String> {
        let no_args = Vec::<String>::new();
        let flags = match &self.version {
            Some(v) => {
                let mut v_flags = v.for_value(ver).unwrap_or(Vec::<String>::new());
                let mut all_flags = self.flags.as_ref().unwrap_or(&no_args).clone();
                all_flags.append(&mut v_flags);

                all_flags
            }
            None => self.flags.as_ref().unwrap_or(&no_args).to_vec(),
        };

        let command = match self.position {
            KeyPosition::First => process::Command::new(&self.command)
                .arg(key)
                .args(flags)
                .output(),
            KeyPosition::Last => process::Command::new(&self.command)
                .args(flags)
                .arg(key)
                .output(),
        };

        match command {
            Ok(result) => match String::from_utf8(result.stdout) {
                Ok(out) => Some(out),
                Err(_) => None,
            },
            Err(_) => None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VersionArg {
    flag: Option<String>,
    #[serde(default)]
    format: VersionFormat,
}

impl VersionArg {
    pub fn for_value(&self, ver: Option<&str>) -> Option<Vec<String>> {
        if let None = ver {
            return None;
        }

        if let Some(flag) = &self.flag {
            match self.format {
                VersionFormat::Disperate => Some(vec![flag.clone(), ver.unwrap().to_owned()]),
                VersionFormat::Concatinate => {
                    Some(vec![format!("{}{}", flag, ver.unwrap().to_owned())])
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum VersionFormat {
    Concatinate,
    Disperate,
}

impl Default for VersionFormat {
    fn default() -> Self {
        VersionFormat::Disperate
    }
}

#[derive(Debug, Deserialize)]
pub enum KeyPosition {
    First,
    Last,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_string() {
        let placeholder = Placeholder::default();
        let regex = format!("{}", placeholder);

        assert_eq!(regex, "(\\{\\{\\s*(?P<source>[^:]+):(?P<label>[^:^\\s]+)(:(?P<version>[^:^\\s]+)\\s*\\}\\}|\\s*\\}\\}))");
    }

    #[test]
    fn versioned_match() {
        let placeholder = Placeholder::default();
        let captures = placeholder.regex().captures("{{ ssm:target:3 }}").unwrap();

        assert_eq!(captures.name("source").unwrap().as_str(), "ssm");
        assert_eq!(captures.name("label").unwrap().as_str(), "target");
        assert_eq!(captures.name("version").unwrap().as_str(), "3");
    }

    #[test]
    fn versionless_match() {
        let placeholder = Placeholder::default();
        let captures = placeholder.regex().captures("{{ ssm:target }}").unwrap();

        assert_eq!(captures.name("source").unwrap().as_str(), "ssm");
        assert_eq!(captures.name("label").unwrap().as_str(), "target");
        assert_eq!(captures.name("version"), None);
    }

    #[test]
    fn user_defined_placeholder() {
        let placeholder = custom_placeholder();
        let captures = placeholder
            .regex()
            .captures("[[ env,target,version ]]")
            .unwrap();

        assert_eq!(captures.name("source").unwrap().as_str(), "env");
        assert_eq!(captures.name("label").unwrap().as_str(), "target");
        assert_eq!(captures.name("version").unwrap().as_str(), "version");
    }

    fn custom_placeholder() -> Placeholder {
        Placeholder {
            opening: "[[".to_string(),
            separator: ",".to_string(),
            closing: "]]".to_string(),
        }
    }

    #[test]
    fn config() {
        let config = Config::new(Some("support/config.json"));
        let placeholder = &config.placeholder;
        let commands = &config.commands;

        assert_eq!(placeholder.opening, "{{");
        assert_eq!(placeholder.separator, ":");
        assert_eq!(placeholder.closing, "}}");
        assert_eq!(commands.len(), 2);
    }
}
