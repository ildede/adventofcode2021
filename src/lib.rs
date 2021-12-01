use std::fs;

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

pub fn run(config: Config) {
    println!("Running puzzle of day {} part {}", config.day, config.part);

    let contents = fs::read_to_string(format!("input/day{}part{}", config.day, config.part))
        .expect("Error reading file");

    let count = count_increasing(contents);
    println!("Count: {:?}", count);
}

fn count_increasing(contents: String) -> usize {
    let mut count: usize = 0;
    let mut prev: usize = 999999999999;
    for line in contents.lines() {
        if line.parse::<usize>().unwrap() > prev {
            count = count + 1;
        }

        prev = line.parse().unwrap();
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::count_increasing;

    #[test]
    fn count_only_increasing() {
        let contents = String::from("100\n101\n102\n");

        assert_eq!(2, count_increasing(contents));
    }

    #[test]
    fn count_only_decreasing() {
        let contents = String::from("100\n99\n50\n");

        assert_eq!(0, count_increasing(contents));
    }

    #[test]
    fn count_mixed() {
        let contents = String::from("100\n110\n120\n90\n80\n100");

        assert_eq!(3, count_increasing(contents));
    }
}
