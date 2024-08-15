use std::{env, process};
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in {}", config.query, config.file_path);
    if let Err(err) = minigrep::run(config){
        println!("Application error: {}", err);
        process::exit(1);
    }
}

