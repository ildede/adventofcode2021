use std::error::Error;
use std::fs;

mod day1;

pub struct Config {
    pub day: u8,
    pub part: u8,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("too many arguments");
        }

        let day = match args[1].parse() {
            Ok(n) => {
                n
            }
            Err(_) => {
                return Err("first argument (day) must be a number");
            }
        };
        let part: u8 = match args[2].parse() {
            Ok(n) => {
                n
            }
            Err(_) => {
                return Err("second argument (part) must be a number");
            }
        };

        Ok(Config { day, part })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Running puzzle of day {} part {}", config.day, config.part);

    let contents: String = fs::read_to_string(format!("input/day{}part{}", config.day, config.part))?;

    let result: String = match config.day {
        1 => day1::solve_puzzle(config.part, contents),
        _ => panic!("invalid day")
    };
    println!("Result: {}", result);

    Ok(())
}
