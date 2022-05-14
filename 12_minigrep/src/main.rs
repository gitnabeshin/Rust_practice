extern crate minigrep;

use std::env;
use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}, {:?}", args, args.len());

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("ERROR: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {

        println!("run error{}", e);
        process::exit(1);
    }
}
