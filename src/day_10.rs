//! # Advent of Code 2021 - Day 10
//!
//! This module contains the solution of the [tenth day's challenges](https://adventofcode.com/2021/day/10).

#[derive(Default, Debug)]
struct BracketCounters {
    round: isize,
    square: isize,
    curly: isize,
    angle: isize,
}

impl BracketCounters {
    fn counter_sum(&self) -> isize {
        self.round + self.square + self.curly + self.angle
    }
}

fn is_corrupted(line: &str) -> Option<char> {
    let mut bracket_counters = BracketCounters::default();
    println!("{:?}", &bracket_counters);
    for c in line.chars() {
        println!("{:?}", &bracket_counters);
        let sum = bracket_counters.counter_sum();
        match c {
            '(' => bracket_counters.round += 1,
            ')' => {
                if bracket_counters.round == 1 && sum == 1 {
                    return Some(')');
                } else {
                    bracket_counters.round -= 1;
                }
            }
            '[' => bracket_counters.square += 1,
            ']' => {
                if bracket_counters.square == 0 {
                    return Some(']');
                } else {
                    bracket_counters.square -= 1;
                }
            }
            '{' => bracket_counters.curly += 1,
            '}' => {
                if bracket_counters.curly == 1 && sum == 1 {
                    return Some('}');
                } else {
                    bracket_counters.curly -= 1;
                }
            }
            '<' => bracket_counters.angle += 1,
            _ => {
                if bracket_counters.angle == 1 && sum == 1 {
                    return Some('>');
                } else {
                    bracket_counters.angle -= 1;
                }
            }
        }
    }
    None
}

pub fn day_10_1(data: &[String]) -> isize {
    data.iter()
        .filter_map(|line| is_corrupted(line))
        .map(|illegal| {
            println!("{:?}", illegal);
            illegal
        })
        .map(|illegal| match illegal {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            _ => 25137,
        })
        .sum()
}

#[cfg(test)]
mod tests {

    // use super::*;

    // #[test]
    // fn test_is_corrupted() {
    //     assert_eq!(is_corrupted("{([(<{}[<>[]}>{[]{[(<()>"), Some('}'));
    // }

    // #[test]
    // fn test_day_10_1() {
    //     let input = vec![
    //         "[({(<(())[]>[[{[]{<()<>>".to_string(),
    //         "[(()[<>])]({[<{<<[]>>(".to_string(),
    //         "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
    //         "(((({<>}<{<{<>}{[]{[]{}".to_string(),
    //         "[[<[([]))<([[{}[[()]]]".to_string(),
    //         "[{[{({}]{}}([{[{{{}}([]".to_string(),
    //         "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
    //         "[<(<(<(<{}))><([]([]()".to_string(),
    //         "<{([([[(<>()){}]>(<<{{".to_string(),
    //         "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    //     ];

    //     assert_eq!(day_10_1(&input), 26397)
    // }
}
