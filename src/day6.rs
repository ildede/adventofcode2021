use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let mut result: Vec<u8> = split_to_numbers(&rows_of_content[0]);
            for i in 0..80 {
                result = wait_one_day(result);
            }
            String::from(result.len().to_string())
        }
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

fn split_to_numbers(list: &String) -> Vec<u8> {
    let split = list.split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect();
    split
}

fn wait_one_day(input: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut new_numbers: Vec<u8> = Vec::new();
    for n in input {
        match n {
            0 => {
                result.push(6);
                new_numbers.push(8);
            }
            n => {
                result.push(n - 1)
            }
        }
    }
    result.append(&mut new_numbers);
    result
}

#[cfg(test)]
mod tests {
    use crate::day6::{solve_puzzle, split_to_numbers, wait_one_day};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("3,4,3,1,2");

        assert_eq!("5934", solve_puzzle(1, contents));
    }

    #[test]
    fn converts_to_list_of_drawn_numbers() {
        let contents = String::from("3,4,3,1,2");

        assert_eq!(vec![3, 4, 3, 1, 2], split_to_numbers(&contents));
    }

    #[test]
    fn it_decrease_each_number_by_one() {
        let input = vec![3, 4, 3, 1, 2];

        assert_eq!(vec![2, 3, 2, 0, 1], wait_one_day(input));
    }

    #[test]
    fn it_decrease_each_number_by_one_and_split_0_as_two_numbers() {
        let input = vec![2, 3, 2, 0, 1];

        assert_eq!(vec![1, 2, 1, 6, 0, 8], wait_one_day(input));
    }

    #[test]
    fn it_keep_new_numbers_to_the_end() {
        let input = vec![0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8];

        assert_eq!(vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8], wait_one_day(input));
    }
}
