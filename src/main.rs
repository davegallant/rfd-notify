mod config;
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
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let config_path = &args[1];
            let config = config::parse(config_path);
            println!("{:?}\n", config);
            let hot_deals = rfd::get_hot_deals()
                .map_err(|err| println!("{:?}", err))
                .ok();
            rfd::parse_hot_deals(hot_deals.unwrap())
        }
        _ => {
            help();
        }
    }
}
