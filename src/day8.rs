use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let x: usize = rows_of_content.iter()
                .map(|r| split_to_row_object(r.to_string()))
                .map(|ro| count_easy_digits(ro))
                .sum();
            println!("Qui cosa arriva {:?}", x);
            String::from(x.to_string())
        }
        2 => unimplemented!("not implemented yet"),
        _ => panic!("invalid part")
    }
}

pub fn split_to_row_object(row: String) -> (Vec<String>, Vec<String>) {
    let mut result: (Vec<String>, Vec<String>) = (vec![String::from(""); 2], vec![String::from(""); 10]);
    let vec: Vec<String> = row.split("|").map(String::from).collect();

    result.0 = vec.clone()[0].split(" ").map(String::from).collect();
    result.1 = vec.clone()[1].split(" ").map(String::from).collect();

    result
}

fn count_easy_digits(input: (Vec<String>, Vec<String>)) -> usize {
    let mut count: usize = 0;
    for el in input.1 {
        match el.len() {
            2 => count += 1,
            4 => count += 1,
            3 => count += 1,
            7 => count += 1,
            _ => {}
        };
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::day8::{count_easy_digits, solve_puzzle, split_to_row_object};

    #[test]
    fn test_puzzle_example_part_one() {
        let contents = String::from("\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
");

        assert_eq!("26", solve_puzzle(1, contents));
    }

    fn it_splits_one_row() {
        let content = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");

        let expected: (Vec<String>, Vec<String>) = (
            vec![String::from("be"), String::from("cfbegad"), String::from("cbdgef"), String::from("fgaecd"), String::from("cgeb"), String::from("fdcge"), String::from("agebfd"), String::from("fecdb"), String::from("fabcd"), String::from("edb")],
            vec![String::from("fdgacbe"), String::from("cefdb"), String::from("cefbgd"), String::from("gcbe")]
        );
        assert_eq!(expected, split_to_row_object(content));
    }

    fn it_counts_easy_digits() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("be"), String::from("cfbegad"), String::from("cbdgef"), String::from("fgaecd"), String::from("cgeb"), String::from("fdcge"), String::from("agebfd"), String::from("fecdb"), String::from("fabcd"), String::from("edb")],
            vec![String::from("fdgacbe"), String::from("cefdb"), String::from("cefbgd"), String::from("gcbe")]
        );

        assert_eq!(2, count_easy_digits(input));
    }
}
