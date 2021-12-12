use crate::utils::{convert_to_vec, split_to_numbers, split_to_usize};

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let numbers = split_to_usize(rows_of_content[0].clone());
            let max = numbers.clone().iter().max().unwrap().clone();
            let mut results: Vec<usize> = vec![0; max as usize];
            for i in 0..results.len() {
                results[i] = numbers.clone().iter().map(|e| {
                    abs_diff(*e, i)
                }).sum();
            }
            String::from(results.iter().min().unwrap().to_string())
        }
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

fn abs_diff(slf: usize, other: usize) -> usize {
    if slf < other {
        other - slf
    } else {
        slf - other
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::solve_puzzle;

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("16,1,2,0,4,2,7,1,2,14");

        assert_eq!("37", solve_puzzle(1, contents));
    }
}
