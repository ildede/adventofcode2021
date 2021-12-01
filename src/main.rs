use std::{env, process};

use adventofcode2021::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(format!("day{:}part{:}", config.day, config.part));
}

struct Config {
    day: u8,
    part: u8,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("too many arguments");
        }

        let day = match args[1].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                return Err("first argument (day) must be a number")
            },
        };
        let part: u8 = match args[2].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                return Err("second argument (part) must be a number")
            },
        };

        Ok(Config { day, part })
    }
}
