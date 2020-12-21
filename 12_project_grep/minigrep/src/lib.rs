use std::fs::File;
use std::error::Error;
use std::io::Read;

pub struct Config { pub query: String, pub filename: String }
impl Config {
    pub fn new(args: &[String]) ->  Result<Config, &'static str> { 
        if args.len() < 3 { return Err("command line args length should be larger than 2"); }
        let config =  Config { query: args[1].clone(), filename: args[2].clone() };
        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}