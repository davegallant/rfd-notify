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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   // Specify path to config
   #[arg(short, long, default_value = "./config.yml")]
   config: String,

   // Specify path to where the embedded database is stored
   #[arg(short, long, default_value = "./deals_db")]
   dbpath: String,

}

fn main() {
    setup_logging();

    debug!("Starting rfd-notify");

    let args = Args::parse();

    debug!("Finding matches...");

    let parsed_config = config::load(&args.config);

    info!("{:?}\n", parsed_config);
    let hot_deals = rfd::get_hot_deals().map_err(|err| error!("{:?}", err)).ok();
    let parsed_deals = rfd::parse_hot_deals(&hot_deals.unwrap());
    rfd::match_deals(
        parsed_deals,
        parsed_config,
        &args.dbpath,
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
