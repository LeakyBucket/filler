extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};

use std::env;

use crate::config::Config;
use crate::schema::Address;

#[derive(Debug)]
pub struct Secret {
    pub name: String,
    pub value: String,
    pub version: Option<String>,
}

#[derive(Debug)]
pub struct SSM {
    pub secret: Option<Secret>,
}

impl SSM {
    pub fn get(placeholder: &Address) -> SSM {
        let client = SsmClient::new(Region::default());
        let req = GetParameterRequest {
            name: placeholder.label.to_string(),
            with_decryption: Some(true),
        };

        match client.get_parameter(req).sync() {
            Err(err) => {
                println!("There was an error fetching {}", placeholder);
                if env::var("FILLER_DEBUG") == Ok("1".to_string()) {
                    println!("{:?}", err);
                }

                SSM { secret: None }
            }
            Ok(res) => match res.parameter {
                None => SSM { secret: None },
                Some(value) => match value.value {
                    None => SSM { secret: None },
                    Some(actual_value) => SSM {
                        secret: Some(Secret {
                            name: placeholder.label.to_owned(),
                            value: actual_value,
                            version: None,
                        }),
                    },
                },
            },
        }
    }
}

#[derive(Debug)]
pub struct Env {
    pub secret: Option<Secret>,
}

impl Env {
    pub fn get(placeholder: &Address) -> Env {
        match env::var(&placeholder.label) {
            Err(_) => {
                println!("No value found for {}", placeholder);
                Env { secret: None }
            }
            Ok(value) => Env {
                secret: Some(Secret {
                    name: placeholder.label.to_string(),
                    value: value,
                    version: None,
                }),
            },
        }
    }
}

#[derive(Debug)]
pub struct Custom {
    pub secret: Option<Secret>,
}

impl Custom {
    pub fn get(placeholder: &Address, config: &Config) -> Custom {
        let secret = match config.command(placeholder.source) {
            Some(command) => command.run(placeholder.label, placeholder.version),
            None => None,
        };

        let version = match placeholder.version {
            Some(ver) => Some(ver.to_owned()),
            None => None,
        };

        match secret {
            None => Custom { secret: None },
            Some(value) => Custom {
                secret: Some(Secret {
                    name: placeholder.label.to_string(),
                    value,
                    version,
                }),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Command, KeyPosition, Placeholder};
    use std::collections::HashMap;

    //#[test]
    //fn ssm_fetch() {
    //    let address = Address {
    //        source: "ssm",
    //        label: "GlenTest",
    //        version: Some("1"),
    //    };
    //    let value = SSM::get(&address);
    //
    //    assert_eq!(value.secret.unwrap().value, "Result".to_string());
    //}

    #[test]
    fn env_fetch() {
        env::set_var("TEST", "Result");
        let address = Address {
            source: "env",
            label: "TEST",
            version: Some("2"),
        };
        let value = Env::get(&address);

        assert_eq!(value.secret.unwrap().value, "Result".to_string());
    }

    #[test]
    fn custom_fetch() {
        let command = Command::new("echo", None, None, KeyPosition::Last);
        let mut commands = HashMap::<String, Command>::new();
        commands.insert("echo".to_owned(), command);
        let address = Address {
            source: "echo",
            label: "TEST",
            version: None,
        };
        let config = Config {
            commands,
            placeholder: Placeholder::default(),
        };
        let value = Custom::get(&address, &config);

        assert_eq!(value.secret.unwrap().value, "TEST\n".to_string());
    }
}
