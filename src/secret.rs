extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};

use std::env;

use crate::schema::Address;
use crate::config::Config;

#[derive(Debug)]
pub struct Secret {
    name: String,
    value: Option<String>,
    version: Option<String>,
}

#[derive(Debug)]
struct SSM {
    secret: Secret
}

impl SSM {
    fn get(placeholder: Address) -> Option<SSM> {
        let client = SsmClient::new(Region::default());
        let req = GetParameterRequest{ name: placeholder.label.clone(), with_decryption: Some(true) };

        match client.get_parameter(req).sync() {
            Err(_err) => {
                println!("There was an error fetching {}", placeholder);

                None
            },
            Ok(res) => {
                let secret = Secret{ name: placeholder.label, value: res.parameter?.value, version: None};

                Some(SSM{secret})
            }
        }
    }
}

#[derive(Debug)]
struct Env {
    secret: Secret
}

impl Env {
    fn get(placeholder: Address) -> Env {
        let value = match env::var(&placeholder.label) {
            Ok(value) => Some(value),
            Err(_) => {
                println!("No value found for {}", placeholder);
                None
            }
        };

        Env {
            secret: Secret {
                name: placeholder.label,
                value: value,
                version: None
            }
        }
    }
}

#[derive(Debug)]
struct Custom {
    secret: Secret
}

impl Custom {
    fn get(placeholder: Address, config: Config) -> Custom {
        let secret = match config.command(placeholder.source.as_str()) {
            Some(command) => command.run(placeholder.label.as_str()),
            None => None
        };

        Custom {
            secret: Secret {
                name: placeholder.label,
                value: secret,
                version: placeholder.version,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Command, Placeholder};
    use std::collections::HashMap;

    #[test]
    fn ssm_fetch() {
        let address = Address{ source: "ssm".to_string(), label: "GlenTest".to_string(), version: Some("1".to_string()) };
        let value = SSM::get(address);

        assert_eq!(value.unwrap().secret.value, Some("Result".to_string()));
    }

    #[test]
    fn env_fetch() {
        let address = Address{ source: "env".to_string(), label: "TEST".to_string(), version: Some("2".to_string()) };
        let value = Env::get(address);

        assert_eq!(value.secret.value, Some("Result".to_string()));
    }

    #[test]
    fn custom_fetch() {
        let address = Address{ source: "credstash".to_string(), label: "test".to_string(), version: Some("8".to_string()) };
        let config = Config{ commands: HashMap::<String, Command>::new(), placeholder: Placeholder::default(), file_name: "".to_string() };
        let value = Custom::get(address, config);

        assert_eq!(value.secret.value, Some("Result".to_string()));
    }
}