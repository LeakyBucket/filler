extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{GetParameterRequest, Ssm, SsmClient};

use std::env;

use crate::schema::Placeholder;
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
    fn get(placeholder: Placeholder, config: Config) -> Option<SSM> {
        let client = SsmClient::new(Region::default());
        let req = GetParameterRequest{ name: placeholder.label, with_decryption: Some(true) };

        match client.get_parameter(req).sync() {
            Err(err) => {
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
    fn get(placeholder: Placeholder, config: Config) -> Env {
        let value = match env::var(placeholder.label) {
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
    fn get(placeholder: Placeholder, config: Config) -> Custom {

    }
}