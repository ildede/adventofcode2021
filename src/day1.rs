use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let vec = convert_to_numbers(convert_to_vec(contents));
    match part {
        1 => count_increasing(vec).to_string(),
        2 => count_increasing(regroup_to_three_measurement(vec)).to_string(),
        _ => panic!("invalid part")
    }
}

fn convert_to_numbers(contents: Vec<String>) -> Vec<usize> {
    let mut vec = Vec::new();
    for line in contents {
        vec.push(line.parse::<usize>().unwrap());
    }
    vec
}

fn regroup_to_three_measurement(vec: Vec<usize>) -> Vec<usize> {
    let mut vec1 = Vec::new();
    for n in 0..vec.len() - 2 {
        vec1.push(vec[n] + vec[n + 1] + vec[n + 2]);
    }
    vec1
}

fn count_increasing(vec: Vec<usize>) -> usize {
    let mut count: usize = 0;

    for n in 1..vec.len() {
        if vec[n] > vec[n - 1] {
            count = count + 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::day1::{convert_to_numbers, count_increasing, regroup_to_three_measurement};

    #[test]
    fn test_puzzle_example_part_two() {
        let contents = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(vec![607, 618, 618, 617, 647, 716, 769, 792], regroup_to_three_measurement(contents));
    }

    #[test]
    fn regroup_three_numbers_input() {
        let contents = vec![10, 20, 30];

        assert_eq!(vec![60], regroup_to_three_measurement(contents))
    }

    #[test]
    fn regroup_four_numbers_input() {
        let contents = vec![10, 20, 30, 40];

        assert_eq!(vec![60, 90], regroup_to_three_measurement(contents))
    }

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(7, count_increasing(contents));
    }

    #[test]
    fn count_only_increasing() {
        let contents = vec![100, 101, 102];

        assert_eq!(2, count_increasing(contents));
    }

    #[test]
    fn count_only_decreasing() {
        let contents = vec![100, 99, 50];

        assert_eq!(0, count_increasing(contents));
    }

    #[test]
    fn count_mixed() {
        let contents = vec![100, 110, 120, 90, 80, 100];

        assert_eq!(3, count_increasing(contents));
    }

    #[test]
    fn conversion_from_string_to_vec() {
        let contents = vec![String::from("199"), String::from("200"), String::from("208"), String::from("210"), String::from("200"), String::from("207"), String::from("240"), String::from("269"), String::from("260"), String::from("263")];

        assert_eq!(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263], convert_to_numbers(contents));
    }
}
