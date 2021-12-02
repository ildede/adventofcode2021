pub fn solve_puzzle(part: u8, contents: String) -> String {
    let vec = convert_to_vec(contents);
    match part {
        1 => {
            let result = find_arrival_point(vec);
            (result.0 * result.1).to_string()
        }
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

fn find_arrival_point(vec: Vec<String>) -> (isize, isize) {
    let mut result: (isize, isize) = (0, 0);
    for x in vec {
        result = apply_instruction(result, x);
    }
    result
}

fn convert_to_vec(contents: String) -> Vec<String> {
    let mut vec = Vec::new();
    for line in contents.lines() {
        vec.push(line.to_string());
    }
    vec
}

fn apply_instruction(actual: (isize, isize), instruction: String) -> (isize, isize) {
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

#[cfg(test)]
mod tests {
    use crate::day2::{apply_instruction, convert_to_vec, find_arrival_point, solve_puzzle};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        assert_eq!("150", solve_puzzle(1, contents));
    }

    #[test]
    fn find_arrival_point_of_puzzle_example_part_one() {
        assert_eq!(
            (15, 10),
            find_arrival_point(vec![String::from("forward 5"), String::from("down 5"), String::from("forward 8"), String::from("up 3"), String::from("down 8"), String::from("forward 2")])
        );
    }

    #[test]
    fn apply_forward() {
        assert_eq!(
            (5, 0),
            apply_instruction((0, 0), String::from("forward 5"))
        );
    }

    #[test]
    fn apply_up() {
        assert_eq!(
            (0, -5),
            apply_instruction((0, 0), String::from("up 5"))
        );
    }

    #[test]
    fn apply_down() {
        assert_eq!(
            (0, 5),
            apply_instruction((0, 0), String::from("down 5"))
        );
    }

    #[test]
    fn conversion_from_string_to_vec() {
        let contents = String::from("forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2");

        assert_eq!(vec!["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"], convert_to_vec(contents));
    }
}
