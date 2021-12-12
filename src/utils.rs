pub fn convert_to_vec(contents: String) -> Vec<String> {
    let mut vec = Vec::new();
    for line in contents.lines() {
        vec.push(line.to_string());
    }
    vec
}

pub fn split_to_numbers(list: String) -> Vec<u8> {
    list.split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect()
}

pub fn split_to_usize(list: String) -> Vec<usize> {
    list.split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::utils::{convert_to_vec, split_to_numbers};

    #[test]
    fn conversion_from_string_to_vec_words_and_numbers() {
        let contents = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        assert_eq!(vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"], convert_to_vec(contents));
    }

    #[test]
    fn conversion_from_string_to_vec_binary_numbers() {
        let contents = String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010");

        assert_eq!(vec!["00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001", "00010", "01010"], convert_to_vec(contents));
    }

    #[test]
    fn conversion_from_string_to_vec_of_u8_numbers() {
        let contents = String::from("3,4,3,1,2");

        assert_eq!(vec![3, 4, 3, 1, 2], split_to_numbers(contents));
    }
}
