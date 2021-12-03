//! # Advent of Code 2021 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2021/day/2).

//use std::io::BufRead;

pub fn day_2(data: &[String]) -> u32 {
    let (horizontal, depth) = data
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|v| (v.0, v.1.parse::<u32>().unwrap()))
        .fold(
            (0, 0),
            |(horizontal, depth), (direction, value)| match direction {
                "up" => (horizontal, depth - value),
                "down" => (horizontal, depth + value),
                _ => (horizontal + value, depth),
            },
        );
    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_1() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        assert_eq!(day_2(&input), 150);
    }
}
