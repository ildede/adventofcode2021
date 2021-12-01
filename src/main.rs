use std::env;

use adventofcode2021::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    run(format!("day{}part{}", config.day, config.part));
}

struct Config {
    day: String,
    part: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let day = args[1].clone();
        let part = args[2].clone();
        Config { day, part }
    }
}
