use std::env;

use adventofcode2021::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1];
    let part = &args[2];

    run(format!("day{}part{}", day, part));
}
