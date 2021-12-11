use std::error::Error;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;

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

    let contents: String = fs::read_to_string(format!("input/day{}input", config.day))?;

    let result: String = match config.day {
        1 => day1::solve_puzzle(config.part, contents),
        2 => day2::solve_puzzle(config.part, contents),
        3 => day3::solve_puzzle(config.part, contents),
        4 => day4::solve_puzzle(config.part, contents),
        5 => day5::solve_puzzle(config.part, contents),
        6 => day6::solve_puzzle(config.part, contents),
        7 => day7::solve_puzzle(config.part, contents),
        8 => day8::solve_puzzle(config.part, contents),
        9 => day9::solve_puzzle(config.part, contents),
        10 => day10::solve_puzzle(config.part, contents),
        11 => day11::solve_puzzle(config.part, contents),
        12 => day12::solve_puzzle(config.part, contents),
        13 => day13::solve_puzzle(config.part, contents),
        14 => day14::solve_puzzle(config.part, contents),
        15 => day15::solve_puzzle(config.part, contents),
        16 => day16::solve_puzzle(config.part, contents),
        17 => day17::solve_puzzle(config.part, contents),
        18 => day18::solve_puzzle(config.part, contents),
        19 => day19::solve_puzzle(config.part, contents),
        20 => day20::solve_puzzle(config.part, contents),
        21 => day21::solve_puzzle(config.part, contents),
        22 => day22::solve_puzzle(config.part, contents),
        23 => day23::solve_puzzle(config.part, contents),
        24 => day24::solve_puzzle(config.part, contents),
        25 => day25::solve_puzzle(config.part, contents),
        _ => panic!("invalid day")
    };
    println!("Result: {}", result);

    Ok(())
}
