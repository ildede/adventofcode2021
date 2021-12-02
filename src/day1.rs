pub fn solve_puzzle(part: u8, contents: String) -> String {
    match part {
        1 => count_increasing(contents).to_string(),
        _ => panic!("invalid part")
    }
}

pub fn count_increasing(contents: String) -> usize {
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
    use crate::day1::count_increasing;

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
