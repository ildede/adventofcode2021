use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let vec = convert_to_vec(contents);
    match part {
        1 => {
            let result = find_arrival_point(vec);
            (result.0 * result.1).to_string()
        }
        2 => {
            let result = find_target_point(vec);
            (result.0 * result.1).to_string()
        }
        _ => panic!("invalid part")
    }
}

fn find_arrival_point(vec: Vec<String>) -> (isize, isize) {
    let mut result: (isize, isize) = (0, 0);
    for x in vec {
        result = apply_instruction_to_arrival(result, x);
    }
    result
}

fn find_target_point(vec: Vec<String>) -> (isize, isize, isize) {
    let mut result: (isize, isize, isize) = (0, 0, 0);
    for x in vec {
        result = apply_instruction_to_target(result, x);
    }
    result
}

fn apply_instruction_to_arrival(actual: (isize, isize), instruction: String) -> (isize, isize) {
    let split = instruction.split(" ");
    let vec: Vec<&str> = split.collect();
    if vec[0] == "forward" {
        return (actual.0 + vec[1].parse::<isize>().unwrap(), actual.1);
    }
    if vec[0] == "down" {
        return (actual.0, actual.1 + vec[1].parse::<isize>().unwrap());
    }
    if vec[0] == "up" {
        return (actual.0, actual.1 - vec[1].parse::<isize>().unwrap());
    }
    actual
}

fn apply_instruction_to_target(actual: (isize, isize, isize), instruction: String) -> (isize, isize, isize) {
    let split = instruction.split(" ");
    let vec: Vec<&str> = split.collect();
    if vec[0] == "forward" {
        return (actual.0 + vec[1].parse::<isize>().unwrap(), actual.1 + (vec[1].parse::<isize>().unwrap() * actual.2), actual.2);
    }
    if vec[0] == "down" {
        return (actual.0, actual.1, actual.2 + vec[1].parse::<isize>().unwrap());
    }
    if vec[0] == "up" {
        return (actual.0, actual.1, actual.2 - vec[1].parse::<isize>().unwrap());
    }
    actual
}

#[cfg(test)]
mod tests {
    use crate::day2::{apply_instruction_to_arrival, apply_instruction_to_target, find_arrival_point, find_target_point, solve_puzzle};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        assert_eq!("150", solve_puzzle(1, contents));
    }

    #[test]
    fn test_puzzle_example_part_two() {
        let contents = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        assert_eq!("900", solve_puzzle(2, contents));
    }

    #[test]
    fn find_arrival_point_of_puzzle_example_part_one() {
        assert_eq!(
            (15, 10),
            find_arrival_point(vec![String::from("forward 5"), String::from("down 5"), String::from("forward 8"), String::from("up 3"), String::from("down 8"), String::from("forward 2")])
        );
    }

    #[test]
    fn find_target_point_of_puzzle_example_part_two() {
        assert_eq!(
            (15, 60, 10),
            find_target_point(vec![String::from("forward 5"), String::from("down 5"), String::from("forward 8"), String::from("up 3"), String::from("down 8"), String::from("forward 2")])
        );
    }

    #[test]
    fn apply_forward_to_arrival() {
        assert_eq!(
            (5, 0),
            apply_instruction_to_arrival((0, 0), String::from("forward 5"))
        );
    }

    #[test]
    fn apply_up_to_arrival() {
        assert_eq!(
            (0, -5),
            apply_instruction_to_arrival((0, 0), String::from("up 5"))
        );
    }

    #[test]
    fn apply_down_to_arrival() {
        assert_eq!(
            (0, 5),
            apply_instruction_to_arrival((0, 0), String::from("down 5"))
        );
    }

    #[test]
    fn apply_forward_to_target_with_zero_aim() {
        assert_eq!(
            (5, 0, 0),
            apply_instruction_to_target((0, 0, 0), String::from("forward 5"))
        );
    }

    #[test]
    fn apply_forward_to_target_with_no_zero_aim() {
        assert_eq!(
            (5, 50, 10),
            apply_instruction_to_target((0, 0, 10), String::from("forward 5"))
        );
    }

    #[test]
    fn apply_up_to_target() {
        assert_eq!(
            (0, 0, -5),
            apply_instruction_to_target((0, 0, 0), String::from("up 5"))
        );
    }

    #[test]
    fn apply_down_to_target() {
        assert_eq!(
            (0, 0, 5),
            apply_instruction_to_target((0, 0, 0), String::from("down 5"))
        );
    }
}
