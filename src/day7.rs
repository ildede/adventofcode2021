use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => unimplemented!("not implemented yet"),
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

#[cfg(test)]
mod tests {

}
