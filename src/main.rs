use std::{
    env,
    process
};
use minigrep::Config;

/*
when running with Cargo
$ cargo run -- frog poem.txt
need the '--' to pass the remaining as parameters to our binary
*/
fn main() {
    // env::args() panics with invalid unicode characters
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args)
        // vertical pipes is a closure |err|
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}"); // easy enough to send to stderr
            process::exit(1)
        });

    println!("Searching for '{}'", cfg.query);
    println!("In file '{}'", cfg.file);

    // check for errors on run
    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
