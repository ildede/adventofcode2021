use crate::utils::{convert_to_vec, split_to_numbers};

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let (numbers, boards) = to_full_game(&rows_of_content);
            let (winning_number, winning_board) = get_first_winning_board(numbers.clone(), boards.clone());
            get_score_of_board(winning_number, winning_board, numbers).to_string()
        },
        2 => {
            let (numbers, boards) = to_full_game(&rows_of_content);
            let (winning_number, winning_board) = get_last_winning_board(numbers.clone(), boards);
            get_score_of_board(winning_number, winning_board, numbers).to_string()
        },
        _ => panic!("invalid part")
    }
}

fn get_score_of_board(winning_number: u8, winning_board: Vec<Vec<u8>>, numbers: Vec<u8>) -> usize {
    let mut result: usize = 0;
    let mut extracted_numbers = Vec::new();
    for number in numbers {
        if number != winning_number {
            extracted_numbers.push(number);
        } else {
            extracted_numbers.push(number);
            break;
        }
    }
    for row in winning_board {
        for x in row {
            if extracted_numbers.contains(&x) {
                ();
            } else {
                result = result + (x as usize);
            }
        }
    }
    result = result * (winning_number as usize);
    result
}

fn get_first_winning_board(numbers: Vec<u8>, boards: Vec<Vec<Vec<u8>>>) -> (u8, Vec<Vec<u8>>) {
    let mut min_rounds = 9999999;
    let mut best_board: Vec<Vec<u8>> = Vec::new();
    for board in boards {
        let round = round_to_win(board.clone(), numbers.clone());
        match round {
            Some(c) => {
                if c < min_rounds {
                    min_rounds = c;
                    best_board = board;
                }
            },
            None => ()
        }
    }
    (numbers[min_rounds-1], best_board)
}

fn get_last_winning_board(numbers: Vec<u8>, boards: Vec<Vec<Vec<u8>>>) -> (u8, Vec<Vec<u8>>) {
    let mut min_rounds = 0;
    let mut worst_board: Vec<Vec<u8>> = Vec::new();
    for board in boards {
        let round = round_to_win(board.clone(), numbers.clone());
        match round {
            Some(c) => {
                if c > min_rounds {
                    min_rounds = c;
                    worst_board = board;
                }
            },
            None => ()
        }
    }
    (numbers[min_rounds-1], worst_board)
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

fn to_bingo_boards(rows: &Vec<String>) -> Vec<Vec<Vec<u8>>> {
    let mut boards: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut row_count: u8 = 0;
    let mut tmp: Vec<String> = Vec::new();
    for r in 2..rows.len() {
        if row_count < 4 {
            tmp.push(rows[r].clone());
            row_count = row_count + 1;
        } else if row_count == 4 {
            tmp.push(rows[r].clone());
            row_count = row_count + 1;
            boards.push(to_bingo_board(tmp.clone()));
        } else if row_count == 5 {
            row_count = 0;
            tmp = Vec::new();
        }
    }
    boards
}

fn to_full_game(rows: &Vec<String>) -> (Vec<u8>, Vec<Vec<Vec<u8>>>) {
    (
        split_to_numbers(rows[0].clone()),
        to_bingo_boards(rows)
    )
}

fn round_to_win(input_board: Vec<Vec<u8>>, numbers: Vec<u8>) -> Option<usize> {
    let mut extracted_numbers: Vec<u8> = Vec::new();
    let mut horizontal_round_to_win: Option<usize> = None;
    for n in &numbers {
        extracted_numbers.push(n.clone());
        if input_board.iter().any(|hl| hl.iter().all(|e| extracted_numbers.contains(e))) {
            horizontal_round_to_win = Some(extracted_numbers.len());
            break;
        }
    }
    extracted_numbers = Vec::new();
    let mut vertical_round_to_win: Option<usize> = None;
    'outer_loop: for n in &numbers {
        extracted_numbers.push(n.clone());
        for i in 0..5 {
            if input_board.iter().all(|hl| extracted_numbers.contains(&hl[i])) {
                vertical_round_to_win = Some(extracted_numbers.len());
                break 'outer_loop;
            }
        }
    }
    if horizontal_round_to_win.is_some() && vertical_round_to_win.is_some() {
        let horizontal = horizontal_round_to_win.unwrap();
        let vertical = vertical_round_to_win.unwrap();
        if horizontal <= vertical {
            Some(horizontal)
        } else {
            Some(vertical)
        }
    } else if horizontal_round_to_win.is_some() {
        horizontal_round_to_win
    } else if vertical_round_to_win.is_some() {
        vertical_round_to_win
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::{get_first_winning_board, get_score_of_board, round_to_win, solve_puzzle, to_bingo_board};

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
        );
    }

    #[test]
    fn round_to_win_on_horizontal_line() {
        let input_board: Vec<Vec<u8>> = vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7]
        ];
        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];

        assert_eq!(Some(12), round_to_win(input_board, numbers));
    }

    #[test]
    fn round_to_win_on_vertical_line() {
        let input_board: Vec<Vec<u8>> = vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7]
        ];
        let numbers = vec![14, 10, 18, 22, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(Some(5), round_to_win(input_board, numbers));
    }

    #[test]
    fn return_min_round_to_win_with_both_vertical_and_horizontal_winning() {
        let input_board: Vec<Vec<u8>> = vec![
            vec![14, 3, 4, 5, 6],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7]
        ];
        let numbers = vec![14, 10, 18, 22, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(Some(5), round_to_win(input_board, numbers));
    }

    #[test]
    fn round_to_win_on_last_vertical_line() {
        let input_board: Vec<Vec<u8>> = vec![
            vec![78, 80, 98, 62, 87],
            vec![90, 53, 91, 81, 23],
            vec![46, 15, 4, 63, 74],
            vec![30, 6, 47, 64, 44],
            vec![12, 45, 95, 68, 99]
        ];
        let numbers = vec![87,7,82,21,47,88,12,71,24,35,10,90,4,97,30,55,36,74,19,50,23,46,13,44,69,27,2,0,37,33,99,49,77,15,89,98,31,51,22,96,73,94,95,18,52,78,32,83,85,54,75,84,59,25,76,45,20,48,9,28,39,70,63,56,5,68,61,26,58,92,67,53,43,62,17,81,80,66,91,93,41,64,14,8,57,38,34,16,42,11,86,72,40,65,79,6,3,29,60,1];

        assert_eq!(Some(31), round_to_win(input_board, numbers));
    }

    #[test]
    fn get_first_winning_board_from_example() {
        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
        let board_0: Vec<Vec<u8>> = vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19]
        ];
        let board_1: Vec<Vec<u8>> = vec![
            vec![3, 15, 0, 2, 22],
            vec![9, 18, 13, 17, 5],
            vec![19, 8, 7, 25, 23],
            vec![20, 11, 10, 24, 4],
            vec![14, 21, 16, 12, 6],
        ];
        let board_2: Vec<Vec<u8>> = vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7]
        ];

        assert_eq!((24, board_2.clone()), get_first_winning_board(numbers, vec![board_0.clone(), board_1.clone(), board_2.clone()]));
    }

    #[test]
    fn calculate_winning_score() {
        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
        let board: Vec<Vec<u8>> = vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7]
        ];

        assert_eq!(4512, get_score_of_board(24, board, numbers));
    }
}
