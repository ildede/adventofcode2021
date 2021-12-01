use std::env;

use adventofcode2021::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    run(filename);
}
