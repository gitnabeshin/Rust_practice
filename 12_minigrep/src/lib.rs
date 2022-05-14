use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            // panic!("you need 2 params p1: query, p2: filename");
            return Err("you need 2 params [p1: query, p2: filename]");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    println!("1{:?}, 2{:?}", config.query, config.filename);

    // let mut f = File::open(config.filename).expect("file not found.");
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    // f.read_to_string(&mut contents).expect("read error.");
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
