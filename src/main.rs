extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate crypto;
mod config;
mod db;
mod mail;
mod rfd;

use std::env;

fn help() {
    println!(
        "usage:\n
rfd-notify <config-toml>
    Specify the filepath of the config."
    );
}

fn main() {
    pretty_env_logger::init();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let config_path = &args[1];
            let config = config::parse(config_path);
            debug!("{:?}\n", config);
            let hot_deals = rfd::get_hot_deals().map_err(|err| error!("{:?}", err)).ok();
            let parsed_deals = rfd::parse_hot_deals(&hot_deals.unwrap());
            rfd::match_deals(parsed_deals, config)
        }
        _ => {
            help();
        }
    }
    info!("Complete")
}
