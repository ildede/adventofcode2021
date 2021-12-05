use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    let vec = convert_to_bitsum_vec(&rows_of_content);
    match part {
        1 => {
            (get_gamma_rate(&vec) * get_epsilon_rate(&vec)).to_string()
        },
        2 => {
            (get_oxygen_generator_rating(&rows_of_content) * get_co2_scrubber_rating(&rows_of_content)).to_string()
        },
        _ => panic!("invalid part")
    }
}


fn convert_to_bitsum_vec(contents: &Vec<String>) -> Vec<(u32, u32)> {
    let mut vec: Vec<(u32, u32)> = vec![(0, 0); contents[0].len()];
    for line in contents {
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

fn get_oxygen_generator_rating(vec: &Vec<String>) -> usize {
    let mut cloned = vec.clone();
    for n in 0..vec[0].chars().count() {
        let most_common_value = most_common_value_at_position(&cloned, n);
        cloned.retain(|e| e.chars().nth(n).unwrap() == most_common_value);
        if cloned.len() == 1 {
            break;
        }
    }
    let intval = usize::from_str_radix(cloned[0].as_str(), 2).unwrap();
    intval
}

fn get_co2_scrubber_rating(vec: &Vec<String>) -> usize {
    let mut cloned = vec.clone();
    for n in 0..vec[0].chars().count() {
        let least_common_value = least_common_value_at_position(&cloned, n);
        cloned.retain(|e| e.chars().nth(n).unwrap() == least_common_value);
        if cloned.len() == 1 {
            break;
        }
    }
    let intval = usize::from_str_radix(cloned[0].as_str(), 2).unwrap();
    intval
}

fn most_common_value_at_position(vec: &Vec<String>, position: usize) -> char {
    let mut zero_count: f32 = 0.0;
    for line in vec {
        if line.chars().nth(position).unwrap() == '0' {
            zero_count = zero_count + 1.0;
        }
    }
    if zero_count > (vec.len() as f32 / 2.0) {
        '0'
    } else {
        '1'
    }
}

fn least_common_value_at_position(vec: &Vec<String>, position: usize) -> char {
    let mut one_count: f32 = 0.0;
    for line in vec {
        if line.chars().nth(position).unwrap() == '1' {
            one_count = one_count + 1.0;
        }
    }
    if one_count < (vec.len() as f32 / 2.0) {
        '1'
    } else {
        '0'
    }
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
    use crate::day3::{convert_to_bitsum_vec, get_co2_scrubber_rating, get_epsilon_rate, get_gamma_rate, get_oxygen_generator_rating, least_common_value_at_position, most_common_value_at_position, solve_puzzle};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");

        assert_eq!("198", solve_puzzle(1, contents));
    }

    #[test]
    fn test_puzzle_example_part_two() {
        let contents = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");

        assert_eq!("230", solve_puzzle(2, contents));
    }

    #[test]
    fn find_oxygen_generator_rating_by_bit_criteria_example_part_two() {
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!(23, get_oxygen_generator_rating(&contents))
    }

    #[test]
    fn find_co2_scrubber_rating_by_bit_criteria_example_part_two() {
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!(10, get_co2_scrubber_rating(&contents))
    }

    #[test]
    fn find_most_common_value_of_one() {
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!('1', most_common_value_at_position(&contents, 0));
    }

    #[test]
    fn find_most_common_value_of_zero() {
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!('0', most_common_value_at_position(&contents, 1));
    }

    #[test]
    fn find_least_common_value_of_zero() {
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!('0', least_common_value_at_position(&contents, 0));
    }

    #[test]
    fn find_least_common_value_of_one() {
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!('1', least_common_value_at_position(&contents, 1));
    }

    #[test]
    fn find_default_most_common_value_if_count_is_equal() {
        let contents = vec![String::from("00001"), String::from("00011"), String::from("11110"), String::from("11100")];

        assert_eq!('1', most_common_value_at_position(&contents, 0));
        assert_eq!('1', most_common_value_at_position(&contents, 4));
    }

    #[test]
    fn find_default_most_common_value_if_count_is_equal_case_with_two_elements() {
        let contents = vec![String::from("10110"), String::from("10111")];

        assert_eq!('1', most_common_value_at_position(&contents, 4));
    }

    #[test]
    fn find_default_least_common_value_if_count_is_equal_case_with_two_elements() {
        let contents = vec![String::from("10110"), String::from("10111")];

        assert_eq!('0', least_common_value_at_position(&contents, 4));
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
        let contents = vec![String::from("00100"), String::from("11110"), String::from("10110"), String::from("10111"), String::from("10101"), String::from("01111"), String::from("00111"), String::from("11100"), String::from("10000"), String::from("11001"), String::from("00010"), String::from("01010")];

        assert_eq!(vec![(5, 7), (7, 5), (4, 8), (5, 7), (7, 5)], convert_to_bitsum_vec(&contents));
    }

    #[test]
    fn conversion_from_string_to_bitsum_vec_with_arbitrary_length_of_binary_number() {
        let contents = vec![String::from("110001010110")];

        assert_eq!(
            vec![(0, 1), (0, 1), (1, 0), (1, 0), (1, 0), (0, 1), (1, 0), (0, 1), (1, 0), (0, 1), (0, 1), (1, 0)],
            convert_to_bitsum_vec(&contents)
        );
    }
}
