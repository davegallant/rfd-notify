extern crate toml;
use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub expressions: Vec<String>,
    pub sendgrid: Sendgrid,
}

#[derive(Deserialize, Debug)]
pub struct Sendgrid {
    pub mail_from: String,
    pub mail_to: String,
    pub api_key: String,
}

pub fn parse(filename: &String) -> Config {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the config");
    let config: Config = toml::from_str(&contents).unwrap();

    return config;
}
