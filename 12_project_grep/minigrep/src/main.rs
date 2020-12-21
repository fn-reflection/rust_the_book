use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Config { query: String, filename: String }
impl Config {
    fn new(args: &[String]) ->  Result<Config, &'static str> { 
        if args.len() < 3 { return Err("command line args length should be larger than 2"); }
        let config =  Config { query: args[1].clone(), filename: args[2].clone() };
        Ok(config)
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}


