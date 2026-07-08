use std::{env, fs, process};
use std::error::Error;

#[allow(dead_code, unused)]

pub fn v2() {
    let args: Vec<String> = env::args().collect();

    let config = Config::parse(&args).unwrap_or_else(|e| {
        eprintln!("An error occurred parsing the config: {}", e);  //eprintln! prints to the sterr instead
        process::exit(-1)
    });

    if let Err(e) = run(config) {
        eprintln!("An error occurred during execution: {}", e);
        process::exit(-1)
    }
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config {
    fn parse(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! At least 2 is expected!")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

use minigrep::{search, search_case_insensitive};

fn run(conf: Config) -> Result<(), Box<dyn Error>> {  // this indicates that the function returns a type that implements the Error trait
    let content = fs::read_to_string(conf.file_path)?; // `dyn` stands for "dynamic", because we don't know the exact type

    let res = if conf.ignore_case {
        search_case_insensitive(&conf.query, &content)
    } else {
        search(&conf.query, &content)
    };

    for line in res {
        println!("{line}");
    }

    Ok(())
}