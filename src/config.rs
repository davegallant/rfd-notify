extern crate toml;
use serde_derive::Deserialize;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_config() {
        let file = "./examples/config.toml";
        parse(&file);
    }
}

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

pub fn parse(filename: &str) -> Config {
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|e| panic!("Unable to read configuration file '{}'. {}", filename, e));
    let config: Config = toml::from_str(&contents).unwrap_or_else(|e| {
        panic!(
            "Unable to parse configuration with contents: {}. {}",
            contents, e
        )
    });

    config
}
