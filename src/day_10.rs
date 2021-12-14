//! # Advent of Code 2021 - Day 10
//!
//! This module contains the solution of the [tenth day's challenges](https://adventofcode.com/2021/day/10).
use std::collections::VecDeque;

/// Check if the `line` is corrupted.
fn is_corrupted(line: &str) -> Option<char> {
    let mut queue: VecDeque<_> = VecDeque::new();

    for char in line.chars() {
        match char {
            '(' | '[' | '{' | '<' => queue.push_back(char),
            ')' => {
                if queue.get(queue.len() - 1) == Some(&'(') {
                    queue.pop_back();
                } else {
                    return Some(char);
                }
            }
            ']' => {
                if queue.get(queue.len() - 1) == Some(&'[') {
                    queue.pop_back();
                } else {
                    return Some(char);
                }
            }
            '}' => {
                if queue.get(queue.len() - 1) == Some(&'{') {
                    queue.pop_back();
                } else {
                    return Some(char);
                }
            }
            _ => {
                if queue.get(queue.len() - 1) == Some(&'<') {
                    queue.pop_back();
                } else {
                    return Some(char);
                }
            }
        }
    }

    None
}

/// Find the first illegal character in each line and compute the sum of their scores.
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

    use super::*;

    #[test]
    fn test_is_corrupted() {
        assert_eq!(is_corrupted("{([(<{}[<>[]}>{[]{[(<()>"), Some('}'));
    }

    #[test]
    fn test_day_10_1() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>".to_string(),
            "[(()[<>])]({[<{<<[]>>(".to_string(),
            "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
            "(((({<>}<{<{<>}{[]{[]{}".to_string(),
            "[[<[([]))<([[{}[[()]]]".to_string(),
            "[{[{({}]{}}([{[{{{}}([]".to_string(),
            "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
            "[<(<(<(<{}))><([]([]()".to_string(),
            "<{([([[(<>()){}]>(<<{{".to_string(),
            "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
        ];

        assert_eq!(day_10_1(&input), 26397)
    }
}
