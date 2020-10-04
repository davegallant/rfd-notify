use std::env;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate clap;
extern crate crypto;
mod config;
mod db;
mod mail;
mod rfd;

use clap::{App, Arg};

fn main() {
    setup_logging();

    debug!("Starting rfd-notify");

    let app = App::new("rfd-notify")
        .version("0.1.2")
        .about("Send emails based on regular expressions")
        .args(&[
            Arg::with_name("config")
                .required(true)
                .takes_value(true)
                .short("c")
                .help("Specify path to config")
                .long("config"),
            Arg::with_name("dbpath")
                .default_value("./deals_db")
                .takes_value(true)
                .short("d")
                .help("Specify path to where the embedded database is stored")
                .long("dbpath"),
        ]);

    debug!("Finding matches...");

    let matches = app.get_matches();

    let config = matches.value_of("config").unwrap();
    let parsed_config = config::parse(config);

    info!("{:?}\n", parsed_config);
    let hot_deals = rfd::get_hot_deals().map_err(|err| error!("{:?}", err)).ok();
    let parsed_deals = rfd::parse_hot_deals(&hot_deals.unwrap());
    rfd::match_deals(
        parsed_deals,
        parsed_config,
        matches.value_of("dbpath").unwrap(),
    );
    info!("Complete")
}

fn setup_logging() {
    debug!("Setting up logging.");

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    debug!("{} is set to {:?}", "RUST_LOG", env::var("RUST_LOG"));

    pretty_env_logger::init()
}
