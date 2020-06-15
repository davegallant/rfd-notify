extern crate toml;
use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    keywords: Vec<String>,
    sendgrid: Sendgrid,
}

#[derive(Deserialize, Debug)]
struct Sendgrid {
    mail_from: String,
    mail_to: String,
    api_key: String,
}

pub fn parse(filename: &String) -> Config {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the config");
    let config: Config = toml::from_str(&contents).unwrap();

    println!("{:?}\n", config);
    return config;
}
