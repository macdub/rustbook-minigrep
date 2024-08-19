use std::{
    env,
    process
};
use minigrep::Config;

fn main() {
    // env::args() panics with invalid unicode characters
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args)
        // vertical pipes is a closure |err|
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1)
        });

    println!("Searching for '{}'", cfg.query);
    println!("In file '{}'", cfg.file);

    // check for errors on run
    if let Err(e) = minigrep::run(cfg) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
