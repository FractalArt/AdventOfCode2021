//! # Advent of Code 2021 - Day 6
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2021/day/6).
use std::collections::VecDeque;

/// Compute the number of lanternfish after a given number of `days`.
pub fn day_6(data: &[String], days: usize) -> usize {
    let mut input: VecDeque<usize> = data.iter()
        .flat_map(|v| v.split(','))
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    for _ in 0..days {
        for i in 0..input.len() {
            if input[i] > 0 { input[i] -= 1; } else {
                input[i] = 6;
                input.push_back(8);
            }
        }
    }

    input.len()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_6_1() {
        let input = vec!["3,4,3,1,2".to_string()];
        assert_eq!(day_6(&input, 80), 5934);
    }

    #[test]
    fn test_day_6_2() {
        let input = vec!["3,4,3,1,2".to_string()];
        assert_eq!(day_6(&input, 256), 26984457539);
    }

}