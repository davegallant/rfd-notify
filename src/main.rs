mod config;
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
            println!("{}\n", config_path);
            config::parse(config_path);
        }
        _ => {
            help();
        }
    }
}
