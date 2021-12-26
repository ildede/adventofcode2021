use crate::utils::convert_to_vec;

pub fn solve_puzzle(part: u8, contents: String) -> String {
    let rows_of_content = convert_to_vec(contents);
    match part {
        1 => {
            let x: usize = rows_of_content.iter()
                .map(|r| split_to_row_object(r.to_string()))
                .map(|ro| count_easy_digits(ro))
                .sum();
            String::from(x.to_string())
        }
        2 => {
            let x: usize = rows_of_content.iter()
                .map(|r| split_to_row_object(r.to_string()))
                .map(|ro| decode_digit(ro))
                .sum();
            String::from(x.to_string())
        }
        _ => panic!("invalid part")
    }
}

pub fn split_to_row_object(row: String) -> (Vec<String>, Vec<String>) {
    let mut result: (Vec<String>, Vec<String>) = (vec![String::from(""); 2], vec![String::from(""); 10]);
    let vec: Vec<String> = row.split("|").map(String::from).collect();

    result.0 = vec.clone()[0].split(" ").map(String::from).filter(|s| s != "").collect();
    result.1 = vec.clone()[1].split(" ").map(String::from).filter(|s| s != "").collect();

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

fn decode_digit(input: (Vec<String>, Vec<String>)) -> usize {
    let one = sort_chars(find_one(input.0.clone()));
    let four = sort_chars(find_four(input.0.clone()));
    let seven = sort_chars(find_seven(input.0.clone()));
    let eight = sort_chars(find_eight(input.0.clone()));

    let two = sort_chars(find_two(input.0.clone(), &*one, &*four));
    let three = sort_chars(find_three(input.0.clone(), &*one));
    let five = sort_chars(find_five(input.0.clone(), &*one, &*four));
    let six = sort_chars(find_six(input.0.clone(), &*one, &*four));
    let nine = sort_chars(find_nine(input.0.clone(), &*one, &*four));
    let zero = sort_chars(find_zero(input.0.clone(), &*one, &*four));

    // println!("{:?}", &five.matches(input.1[0]));
    let first = match &sort_chars(input.1[0].clone()) {
        x if one == String::from(x) => 1,
        x if two == String::from(x) => 2,
        x if three == String::from(x) => 3,
        x if four == String::from(x) => 4,
        x if five == String::from(x) => 5,
        x if six == String::from(x) => 6,
        x if seven == String::from(x) => 7,
        x if eight == String::from(x) => 8,
        x if nine == String::from(x) => 9,
        x if zero == String::from(x) => 0,
        _ => unreachable!()
    };
    let second = match &sort_chars(input.1[1].clone()) {
        x if one == String::from(x) => 1,
        x if two == String::from(x) => 2,
        x if three == String::from(x) => 3,
        x if four == String::from(x) => 4,
        x if five == String::from(x) => 5,
        x if six == String::from(x) => 6,
        x if seven == String::from(x) => 7,
        x if eight == String::from(x) => 8,
        x if nine == String::from(x) => 9,
        x if zero == String::from(x) => 0,
        _ => unreachable!()
    };
    let third = match &sort_chars(input.1[2].clone()) {
        x if one == String::from(x) => 1,
        x if two == String::from(x) => 2,
        x if three == String::from(x) => 3,
        x if four == String::from(x) => 4,
        x if five == String::from(x) => 5,
        x if six == String::from(x) => 6,
        x if seven == String::from(x) => 7,
        x if eight == String::from(x) => 8,
        x if nine == String::from(x) => 9,
        x if zero == String::from(x) => 0,
        _ => unreachable!()
    };
    let fourth = match &sort_chars(input.1[3].clone()) {
        x if one == String::from(x) => 1,
        x if two == String::from(x) => 2,
        x if three == String::from(x) => 3,
        x if four == String::from(x) => 4,
        x if five == String::from(x) => 5,
        x if six == String::from(x) => 6,
        x if seven == String::from(x) => 7,
        x if eight == String::from(x) => 8,
        x if nine == String::from(x) => 9,
        x if zero == String::from(x) => 0,
        _ => unreachable!()
    };

    first * 1000 + second * 100 + third * 10 + fourth
}

fn sort_chars(s: String) -> String {
    let mut l: Vec<char> = s.chars().collect();
    l.sort_unstable();
    let j: String = l.into_iter().collect();
    j
}

fn find_one(input: Vec<String>) -> String {
    input.iter().find(|e| e.len() == 2).unwrap().to_string()
}

fn find_two(input: Vec<String>, one: &str, four: &str) -> String {
    input.iter()
        .filter(|e| e.len() == 5)
        .find(|e| e.chars().filter(|&c| one.contains(c)).count() == 1
            && e.chars().filter(|&c| four.contains(c)).count() == 2)
        .unwrap()
        .to_string()
}

fn find_three(input: Vec<String>, one: &str) -> String {
    input.iter()
        .filter(|e| e.len() == 5)
        .find(|e| e.chars().filter(|&c| one.contains(c)).count() == 2)
        .unwrap()
        .to_string()
}

fn find_four(input: Vec<String>) -> String {
    input.iter().find(|e| e.len() == 4).unwrap().to_string()
}

fn find_five(input: Vec<String>, one: &str, four: &str) -> String {
    input.iter()
        .filter(|e| e.len() == 5)
        .find(|e| e.chars().filter(|&c| one.contains(c)).count() == 1
            && e.chars().filter(|&c| four.contains(c)).count() == 3)
        .unwrap()
        .to_string()
}

fn find_six(input: Vec<String>, one: &str, four: &str) -> String {
    input.iter()
        .filter(|e| e.len() == 6)
        .find(|e| e.chars().filter(|&c| one.contains(c)).count() == 1
            && e.chars().filter(|&c| four.contains(c)).count() == 3)
        .unwrap()
        .to_string()
}

fn find_seven(input: Vec<String>) -> String {
    input.iter().find(|e| e.len() == 3).unwrap().to_string()
}

fn find_eight(input: Vec<String>) -> String {
    input.iter().find(|e| e.len() == 7).unwrap().to_string()
}

fn find_nine(input: Vec<String>, one: &str, four: &str) -> String {
    input.iter()
        .filter(|e| e.len() == 6)
        .find(|e| e.chars().filter(|&c| one.contains(c)).count() == 2
            && e.chars().filter(|&c| four.contains(c)).count() == 4)
        .unwrap()
        .to_string()
}

fn find_zero(input: Vec<String>, one: &str, four: &str) -> String {
    input.iter()
        .filter(|e| e.len() == 6)
        .find(|e| e.chars().filter(|&c| one.contains(c)).count() == 2
            && e.chars().filter(|&c| four.contains(c)).count() == 3)
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::day8::{count_easy_digits, decode_digit, find_eight, find_five, find_four, find_nine, find_one, find_seven, find_six, find_three, find_two, find_zero, solve_puzzle, split_to_row_object};

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

    #[test]
    fn test_puzzle_example_part_two() {
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

        assert_eq!("61229", solve_puzzle(2, contents));
    }

    #[test]
    fn it_splits_one_row() {
        let content = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");

        let expected: (Vec<String>, Vec<String>) = (
            vec![String::from("be"), String::from("cfbegad"), String::from("cbdgef"), String::from("fgaecd"), String::from("cgeb"), String::from("fdcge"), String::from("agebfd"), String::from("fecdb"), String::from("fabcd"), String::from("edb")],
            vec![String::from("fdgacbe"), String::from("cefdb"), String::from("cefbgd"), String::from("gcbe")]
        );
        assert_eq!(expected, split_to_row_object(content));
    }

    #[test]
    fn it_counts_easy_digits() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("be"), String::from("cfbegad"), String::from("cbdgef"), String::from("fgaecd"), String::from("cgeb"), String::from("fdcge"), String::from("agebfd"), String::from("fecdb"), String::from("fabcd"), String::from("edb")],
            vec![String::from("fdgacbe"), String::from("cefdb"), String::from("cefbgd"), String::from("gcbe")]
        );

        assert_eq!(2, count_easy_digits(input));
    }

    #[test]
    fn it_decode_digits() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!(5353, decode_digit(input));
    }

    #[test]
    fn it_find_one() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("ab", find_one(input.0));
    }

    #[test]
    fn it_find_two() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("gcdfa", find_two(input.0, "ab", "eafb"));
    }

    #[test]
    fn it_find_three() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("fbcad", find_three(input.0, "ab"));
    }

    #[test]
    fn it_find_four() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("eafb", find_four(input.0));
    }

    #[test]
    fn it_find_five() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("cdfbe", find_five(input.0, "ab", "eafb"));
    }

    #[test]
    fn it_find_six() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("cdfgeb", find_six(input.0, "ab", "eafb"));
    }

    #[test]
    fn it_find_seven() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("dab", find_seven(input.0));
    }

    #[test]
    fn it_find_eight() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("acedgfb", find_eight(input.0));
    }

    #[test]
    fn it_find_nine() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("cefabd", find_nine(input.0, "ab", "eafb"));
    }

    #[test]
    fn it_find_zero() {
        let input: (Vec<String>, Vec<String>) = (
            vec![String::from("acedgfb"), String::from("cdfbe"), String::from("gcdfa"), String::from("fbcad"), String::from("dab"), String::from("cefabd"), String::from("cdfgeb"), String::from("eafb"), String::from("cagedb"), String::from("ab")],
            vec![String::from("cdfeb"), String::from("fcadb"), String::from("cdfeb"), String::from("cdbaf")]
        );

        assert_eq!("cagedb", find_zero(input.0, "ab", "eafb"));
    }
}
