use crate::config::{Config, Placeholder};
use crate::secret::{Custom, Env, SSM};
use std::fmt;

#[derive(Debug)]
pub struct Context<'ctx> {
    pub placeholder: &'ctx Placeholder,
    pub address: Address<'ctx>,
}

impl Context<'_> {
    pub fn new<'n>(placeholder: &'n Placeholder, src: &'n str, label: &'n str, version: Option<&'n str>) -> Context<'n> {
        Context {
            placeholder,
            address: Address {
                source: src,
                label,
                version
            }
        }
    }

    pub fn evaluate(&self, config: &Config) -> String {
        let result = match self.address.source {
            "ssm" => SSM::get(&self.address).secret,
            "env" => Env::get(&self.address).secret,
            _src => Custom::get(&self.address, config).secret
        };

        if let Some(secret) = result {
            secret.value
        } else {
            match self.address.version {
                None => format!("{} {}{}{} {}", self.placeholder.opening, self.address.source, self.placeholder.separator, self.address.label, self.placeholder.closing),
                Some(ver) => format!("{} {}{}{}{}{} {}", self.placeholder.opening, self.address.source, self.placeholder.separator, self.address.label, self.placeholder.separator, ver, self.placeholder.closing)
            }
        }
    }
}

#[derive(Debug)]
pub struct Address<'addr> {
    pub source: &'addr str,
    pub label: &'addr str,
    pub version: Option<&'addr str>
}

impl fmt::Display for Address<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.version {
            Some(ver) => write!(f, "{}:{}:{}", self.source.to_owned(), self.label.to_owned(), ver.to_owned()),
            None => write!(f, "{}:{}", self.source.to_owned(), self.label.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn sub_with_value() {
        let config = Config::default();
        let placeholder = Placeholder::default();

        assert_eq!(context("TEST", &placeholder).evaluate(&config), "Result".to_string());
    }

    #[test]
    fn sub_no_value_no_version() {
        let config = Config::default();
        let placeholder = Placeholder::default();

        assert_eq!(context("TEST2", &placeholder).evaluate(&config), "{{ env:TEST2 }}")
    }

    #[test]
    fn sub_no_value_with_version () {
        let config = Config::default();
        let placeholder = Placeholder::default();

        assert_eq!(context("TEST2", &placeholder).evaluate(&config), "{{ env:TEST2:3 }}")
    }

    fn context<'c>(label: &'c str, placeholder: &'c Placeholder) -> Context<'c> {
        Context {
            placeholder,
            address: Address {
                source: "env",
                label: label,
                version: None
            }
        }
    }
}