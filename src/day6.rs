use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let mut result: Vec<u8> = split_to_numbers(&rows_of_content[0]);
            (0..80).for_each(|day| {
                println!("Day {}", day);
                wait_one_day(&mut result);
            });
            String::from(&result.len().to_string())
        }
        2 => {
            let numbers: Vec<u8> = split_to_numbers(&rows_of_content[0]);
            let mut vec = map_to_day(numbers);
            (0..256).for_each(|day| {
                vec = wait_one_day_on_map(vec.clone());
            });
            String::from(vec.iter().sum::<usize>().to_string())
        }
        _ => panic!("invalid part")
    }
}

fn split_to_numbers(list: &String) -> Vec<u8> {
    let split = list.split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect();
    split
}

fn wait_one_day(input: &mut Vec<u8>) {
    for i in 0..input.len() {
        let i1 = input[i];
        match i1 {
            0 => {
                input[i] = 6;
                input.push(8);
            }
            n => {
                input[i] = input[i] - 1;
            }
        }
    }
}

fn wait_one_day_on_map(input: Vec<usize>) -> Vec<usize> {
    vec![
        input[1],
        input[2],
        input[3],
        input[4],
        input[5],
        input[6],
        input[7] + input[0],
        input[8],
        input[0],
    ]
}

fn map_to_day(input: Vec<u8>) -> Vec<usize> {
    let mut initial = vec![0; 9];
    input.iter().for_each(|e| initial[*e as usize] += 1);
    initial
}

#[cfg(test)]
mod tests {
    use crate::day6::{map_to_day, solve_puzzle, split_to_numbers, wait_one_day, wait_one_day_on_map};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("3,4,3,1,2");

        assert_eq!("5934", solve_puzzle(1, contents));
    }

    #[test]
    fn test_puzzle_example_part_two() {
        let contents = String::from("3,4,3,1,2");

        assert_eq!("26984457539", solve_puzzle(2, contents));
    }

    #[test]
    fn converts_to_list_of_drawn_numbers() {
        let contents = String::from("3,4,3,1,2");

        assert_eq!(vec![3, 4, 3, 1, 2], split_to_numbers(&contents));
    }

    #[test]
    fn it_decrease_each_number_by_one() {
        let mut input: Vec<u8> = vec![3, 4, 3, 1, 2];

        wait_one_day(&mut input);

        let expected: Vec<u8> = vec![2, 3, 2, 0, 1];
        assert_eq!(expected, input);
    }

    #[test]
    fn it_decrease_each_number_by_one_and_split_0_as_two_numbers() {
        let mut input: Vec<u8> = vec![2, 3, 2, 0, 1];

        wait_one_day(&mut input);

        let expected: Vec<u8> = vec![1, 2, 1, 6, 0, 8];
        assert_eq!(expected, input);
    }

    #[test]
    fn it_keep_new_numbers_to_the_end() {
        let mut input: Vec<u8> = vec![0, 1, 0, 5, 6, 0, 1, 2, 2, 3, 0, 1, 2, 2, 2, 3, 3, 4, 4, 5, 7, 8];

        wait_one_day(&mut input);

        let expected: Vec<u8> = vec![6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8];
        assert_eq!(expected, input);
    }

    #[test]
    fn it_maps_number_8_on_respective_day() {
        let input: Vec<u8> = vec![8, 8, 8];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 3];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_7_on_respective_day() {
        let input: Vec<u8> = vec![7, 7, 7];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 3, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_6_on_respective_day() {
        let input: Vec<u8> = vec![6, 6, 6];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 3, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_5_on_respective_day() {
        let input: Vec<u8> = vec![5, 5, 5];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 3, 0, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_4_on_respective_day() {
        let input: Vec<u8> = vec![4, 4, 4];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 3, 0, 0, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_3_on_respective_day() {
        let input: Vec<u8> = vec![3, 3, 3];

        let expected: Vec<usize> = vec![0, 0, 0, 3, 0, 0, 0, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_2_on_respective_day() {
        let input: Vec<u8> = vec![2, 2, 2];

        let expected: Vec<usize> = vec![0, 0, 3, 0, 0, 0, 0, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_1_on_respective_day() {
        let input: Vec<u8> = vec![1, 1, 1];

        let expected: Vec<usize> = vec![0, 3, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_maps_number_0_on_respective_day() {
        let input: Vec<u8> = vec![0, 0, 0];

        let expected: Vec<usize> = vec![3, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(expected, map_to_day(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_8_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 3];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 3, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_7_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 3, 0];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 3, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_6_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 3, 0, 0];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 3, 0, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_5_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 0, 0, 0, 3, 0, 0, 0];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 3, 0, 0, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_4_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 0, 0, 3, 0, 0, 0, 0];

        let expected: Vec<usize> = vec![0, 0, 0, 3, 0, 0, 0, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_3_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 0, 3, 0, 0, 0, 0, 0];

        let expected: Vec<usize> = vec![0, 0, 3, 0, 0, 0, 0, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_2_on_respective_day() {
        let input: Vec<usize> = vec![0, 0, 3, 0, 0, 0, 0, 0, 0];

        let expected: Vec<usize> = vec![0, 3, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_1_on_respective_day() {
        let input: Vec<usize> = vec![0, 3, 0, 0, 0, 0, 0, 0, 0];

        let expected: Vec<usize> = vec![3, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(expected, wait_one_day_on_map(input))
    }

    #[test]
    fn it_wait_one_day_on_maps_of_number_0_on_respective_day() {
        let input: Vec<usize> = vec![3, 0, 0, 0, 0, 0, 0, 0, 0];

        let expected: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 3, 0, 3];
        assert_eq!(expected, wait_one_day_on_map(input))
    }
}
