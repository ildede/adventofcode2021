use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let drawn_numbers = to_drawn_numbers(&rows_of_content[0]);
            String::from("")
        },
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

fn to_drawn_numbers(list: &String) -> Vec<u8> {
    let split = list.split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    println!("{:?}", split);
    split
}

fn to_bingo_board(input_board: Vec<String>) -> Vec<Vec<u8>> {
    let mut board: Vec<Vec<u8>> = Vec::new();
    for n in 0..5 {
        let mut row: Vec<u8> = Vec::new();
        for number in input_board[n].split_whitespace() {
            row.push(number.parse::<u8>().unwrap())
        }
        board.push(row);
    }
    board
}

#[cfg(test)]
mod tests {
    use crate::day4::{solve_puzzle, to_bingo_board, to_drawn_numbers};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7");

        assert_eq!("4512", solve_puzzle(1, contents));
    }

    #[test]
    fn converts_to_list_of_drawn_numbers() {
        let contents = String::from("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1");

        assert_eq!(
            vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1],
            to_drawn_numbers(&contents)
        )
    }

    #[test]
    fn converts_to_bingo_board() {
        let contents = vec![
            String::from("22 13 17 11  0"),
            String::from(" 8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from(" 6 10  3 18  5"),
            String::from(" 1 12 20 15 19")];

        let expected: Vec<Vec<u8>> = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19]
        ];
        assert_eq!(
            expected,
            to_bingo_board(contents)
        )
    }
}
