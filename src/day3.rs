pub fn solve_puzzle(part: u8, contents: String) -> String {
    let vec = convert_to_vec(contents);
    match part {
        1 => {
            (get_gamma_rate(&vec) * get_epsilon_rate(&vec)).to_string()
        }
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

fn convert_to_vec(contents: String) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = vec![(0, 0); contents.lines().nth(0).unwrap().len()];
    for line in contents.lines() {
        for n in 0..line.len() {
            if line.chars().nth(n).unwrap() == '0' {
                vec[n].0 = vec[n].0 + 1;
            } else {
                vec[n].1 = vec[n].1 + 1;
            }
        }
    }
    vec
}

fn get_gamma_rate(vec: &Vec<(u32, u32)>) -> u32 {
    let mut str  = String::new();
    for n in 0..vec.len() {
        if vec[n].0 < vec[n].1 {
            str.push('1');
        } else {
            str.push('0');
        }
    }
    let intval = isize::from_str_radix(&*str, 2).unwrap();
    intval as u32
}

fn get_epsilon_rate(vec: &Vec<(u32, u32)>) -> u32 {
    let mut str  = String::new();
    for n in 0..vec.len() {
        if vec[n].0 > vec[n].1 {
            str.push('1');
        } else {
            str.push('0');
        }
    }
    let intval = isize::from_str_radix(&*str, 2).unwrap();
    intval as u32
}

#[cfg(test)]
mod tests {
    use crate::day3::{convert_to_vec, get_epsilon_rate, get_gamma_rate, solve_puzzle};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");

        assert_eq!("198", solve_puzzle(1, contents));
    }

    #[ignore]
    #[test]
    fn test_puzzle_example_part_two() {
        let contents = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");

        assert_eq!("", solve_puzzle(2, contents));
    }

    #[test]
    fn get_min_gamma_rate() {
        let input: Vec<(u32, u32)> = vec![(1, 0), (1, 0), (1, 0), (1, 0), (1, 0)];

        assert_eq!(0, get_gamma_rate(&input));
    }

    #[test]
    fn get_min_epsilon_rate() {
        let input: Vec<(u32, u32)> = vec![(0, 1), (0, 1), (0, 1), (0, 1), (0, 1)];

        assert_eq!(0, get_epsilon_rate(&input));
    }

    #[test]
    fn get_max_gamma_rate() {
        let input: Vec<(u32, u32)> = vec![(0, 1), (0, 1), (0, 1), (0, 1), (0, 1)];

        assert_eq!(31, get_gamma_rate(&input));
    }

    #[test]
    fn get_max_epsilon_rate() {
        let input: Vec<(u32, u32)> = vec![(1, 0), (1, 0), (1, 0), (1, 0), (1, 0)];

        assert_eq!(31, get_epsilon_rate(&input));
    }

    #[test]
    fn get_gamma_rate_example_part_one() {
        let input: Vec<(u32, u32)> = vec![(5, 7), (7, 5), (4, 8), (5, 7), (7, 5)];

        assert_eq!(22, get_gamma_rate(&input));
    }

    #[test]
    fn get_epsilon_rate_example_part_one() {
        let input: Vec<(u32, u32)> = vec![(5, 7), (7, 5), (4, 8), (5, 7), (7, 5)];

        assert_eq!(9, get_epsilon_rate(&input));
    }

    #[test]
    fn get_max_gamma_rate_of_long_vec() {
        let input: Vec<(u32, u32)> = vec![(0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (0, 1)];

        assert_eq!(4095, get_gamma_rate(&input));
    }

    #[test]
    fn get_max_epsilon_rate_of_long_vec() {
        let input: Vec<(u32, u32)> = vec![(1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0), (1, 0)];

        assert_eq!(4095, get_epsilon_rate(&input));
    }

    #[test]
    fn conversion_from_string_to_bitsum_vec() {
        let contents = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");

        assert_eq!(vec![(5, 7), (7, 5), (4, 8), (5, 7), (7, 5)], convert_to_vec(contents));
    }

    #[test]
    fn conversion_from_string_to_bitsum_vec_with_arbitrary_length_of_binary_number() {
        let contents = String::from("110001010110");

        assert_eq!(
            vec![(0, 1), (0, 1), (1, 0), (1, 0), (1, 0), (0, 1), (1, 0), (0, 1), (1, 0), (0, 1), (0, 1), (1, 0)],
            convert_to_vec(contents)
        );
    }
}
