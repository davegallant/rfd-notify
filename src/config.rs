extern crate envconfig;
extern crate envconfig_derive;

use serde::Deserialize;

use envconfig::Envconfig;
use std::fs;
use std::vec::Vec;

#[cfg(test)]
extern crate serial_test;
#[cfg(test)]
use serial_test::serial;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    #[serial]
    fn load_config_with_missing_sendgrid_api_key() {
        std::env::remove_var("SENDGRID_API_KEY");
        let file = "./examples/config.yml";
        load(file);
    }

    #[test]
    #[serial]
    fn load_config() {
        let file = "./examples/config.yml";
        std::env::set_var("SENDGRID_API_KEY", "FAKE");
        std::env::set_var("SENDGRID_MAIL_FROM", "notify@rfd-notify.org");
        std::env::set_var("SENDGRID_MAIL_TO", "test@email.com");
        load(file);
    }
}

#[derive(Debug)]
pub struct Config {
    pub expressions: Vec<String>,
    pub sendgrid: SendgridConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConfigFile {
    pub expressions: Vec<String>,
}

#[derive(Envconfig, Debug)]
pub struct SendgridConfig {
    #[envconfig(from = "SENDGRID_MAIL_FROM")]
    pub mail_from: String,

    #[envconfig(from = "SENDGRID_MAIL_TO")]
    pub mail_to: String,

    #[envconfig(from = "SENDGRID_API_KEY")]
    pub api_key: String,
}

pub fn load(filename: &str) -> Config {
    // Initialize expressions from file
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|e| panic!("Unable to read configuration file '{}'. {}", filename, e));

    let config_file: ConfigFile = serde_yaml::from_str(&contents).unwrap();

    // Initialize config from environment variables or terminate the process.
    let sendgrid_config = SendgridConfig::init_from_env().unwrap();

    Config {
        expressions: config_file.expressions,
        sendgrid: sendgrid_config,
    }
}
