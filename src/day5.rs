use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let valid_instructions: Vec<Instruction> = rows_of_content.iter()
                .map(|e| Instruction::from(e))
                .filter(Instruction::is_ortogonal)
                .collect();

            let mut grid = create_empty_grid(find_grid_size(&valid_instructions));
            grid.draw_lines(valid_instructions);

            String::from(grid.count_overlapping_points().to_string())
        }
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

#[derive(PartialEq, Debug)]
struct Instruction {
    start: (u16, u16),
    end: (u16, u16),
}

impl Instruction {
    fn from(input: &str) -> Self {
        let mut split = input.split(" -> ");
        let mut input_start = split.clone().nth(0).unwrap().split(',');
        let mut input_end = split.clone().nth(1).unwrap().split(',');
        Instruction {
            start: (
                input_start.clone().nth(0).unwrap().parse::<u16>().unwrap(),
                input_start.clone().nth(1).unwrap().parse::<u16>().unwrap()
            ),
            end: (
                input_end.clone().nth(0).unwrap().parse::<u16>().unwrap(),
                input_end.clone().nth(1).unwrap().parse::<u16>().unwrap()
            ),
        }
    }

    fn is_ortogonal(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn is_on_x(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn is_on_y(&self) -> bool {
        self.start.1 == self.end.1
    }
}

#[derive(PartialEq, Debug)]
struct Grid {
    grid: Vec<Vec<u16>>,
}

impl Grid {
    fn draw_lines(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions {
            if instruction.is_ortogonal() {
                if instruction.is_on_x() {
                    let start_point_x = instruction.start.0 as usize;
                    let start_point_y = instruction.start.1 as usize;
                    let end_point_y = instruction.end.1 as usize;
                    if start_point_y < end_point_y {
                        for i in start_point_y..=end_point_y {
                            self.grid[i][start_point_x] = self.grid[i][start_point_x] + 1;
                        }
                    } else {
                        for i in end_point_y..=start_point_y {
                            self.grid[i][start_point_x] = self.grid[i][start_point_x] + 1;
                        }
                    }
                }
                if instruction.is_on_y() {
                    let start_point_x = instruction.start.0 as usize;
                    let start_point_y = instruction.start.1 as usize;
                    let end_point_x = instruction.end.0 as usize;
                    if start_point_x < end_point_x {
                        for i in start_point_x..=end_point_x {
                            self.grid[start_point_y][i] = self.grid[start_point_y][i] + 1;
                        }
                    } else {
                        for i in end_point_x..=start_point_x {
                            self.grid[start_point_y][i] = self.grid[start_point_y][i] + 1;
                        }
                    }
                }
            }
        }
    }

    fn print(&self) {
        for row in &self.grid {
            println!("{:?}", row);
        }
    }

    fn count_overlapping_points(&self) -> usize {
        let mut count: usize = 0;
        for row in &self.grid {
            for el in row {
                if el > &1 {
                    count = count + 1;
                }
            }
        }
        count
    }
}

fn create_empty_grid(size: (u16, u16)) -> Grid {
    Grid {
        grid: vec![vec![0; size.0 as usize]; size.1 as usize]
    }
}

fn find_grid_size(instructions: &Vec<Instruction>) -> (u16, u16) {
    let mut width: u16 = 0;
    let mut height: u16 = 0;

    for instruction in instructions {
        if instruction.start.0 > width {
            width = instruction.start.0;
        }
        if instruction.end.0 > width {
            width = instruction.end.0;
        }
        if instruction.start.1 > height {
            height = instruction.start.1;
        }
        if instruction.end.1 > height {
            height = instruction.end.1;
        }
    }

    (width + 1, height + 1)
}

#[cfg(test)]
mod tests {
    use crate::day5::{create_empty_grid, find_grid_size, Grid, Instruction, solve_puzzle};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
");

        assert_eq!("5", solve_puzzle(1, contents));
    }

    #[test]
    fn create_instruction() {
        let input = "0,9 -> 5,9";

        assert_eq!(
            Instruction { start: (0, 9), end: (5, 9) },
            Instruction::from(input)
        )
    }

    #[test]
    fn check_ortogonal_movement_of_instruction() {
        let instruction = Instruction { start: (0, 9), end: (5, 9) };

        assert_eq!(true, instruction.is_ortogonal());
    }

    #[test]
    fn find_grid_size_from_instructions() {
        let instructions = vec![
            Instruction { start: (0, 0), end: (0, 0) },
            Instruction { start: (0, 9), end: (5, 0) },
            Instruction { start: (1, 1), end: (1, 1) },
        ];

        assert_eq!((6, 10), find_grid_size(&instructions));
    }

    #[test]
    fn create_an_empty_grid() {
        let expected = Grid { grid: vec![vec![0; 4]; 4] };

        assert_eq!(expected, create_empty_grid((4, 4)));
    }

    #[test]
    fn apply_single_instructions_from_example() {
        let instructions = vec![
            Instruction { start: (0, 9), end: (5, 9) },
            Instruction { start: (9, 4), end: (3, 4) },
            Instruction { start: (2, 2), end: (2, 1) },
            Instruction { start: (7, 0), end: (7, 4) },
            Instruction { start: (0, 9), end: (2, 9) },
            Instruction { start: (3, 4), end: (1, 4) },
        ];
        let mut initial_grid = create_empty_grid((10, 10));
        let expected = Grid {
            grid: vec![
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
                vec![0, 1, 1, 2, 1, 1, 1, 2, 1, 1],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![2, 2, 2, 1, 1, 1, 0, 0, 0, 0],
            ]
        };

        initial_grid.draw_lines(instructions);
        initial_grid.print();
        assert_eq!(expected, initial_grid)
    }
}
