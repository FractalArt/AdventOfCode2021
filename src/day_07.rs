//! # Advent of Code 2021 - Day 7
//!
//! This module contains the solution of the [seventh day's challenges](https://adventofcode.com/2021/day/7).

/// The sum of the integers from 1 to `x`.
fn gauss(x: isize) -> isize {
    x * (x + 1) / 2
}

/// Compute the minimal amount of fuel required to align the crabs at one horizontal position.
///
/// Moving a crab one horizontal unit costs one fuel. Solution by trying all.
pub fn day_7_1(data: &[String]) -> isize {
    let input: Vec<isize> = data[0]
        .split(',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect();

    (0..*input.iter().max().unwrap())
        .map(|pos| input.iter().map(|x| (x - pos).abs()).sum::<isize>())
        .min()
        .unwrap()
}

/// Compute the minimal amount of fuel required to align the crabs at one horizontal position.
///
/// Moving a crab costs increasingly more fuel. The first move costs one fuel, t
/// the second move costs two fuel e.t.c.
pub fn day_7_2(data: &[String]) -> isize {
    let input: Vec<isize> = data[0]
        .split(',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect();

    (0..*input.iter().max().unwrap())
        .map(|pos| input.iter().map(|x| gauss((x - pos).abs())).sum::<isize>())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_7_1() {
        let input = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        assert_eq!(day_7_1(&input), 37);
    }

    #[test]
    fn test_day_7_2() {
        let input = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        assert_eq!(day_7_2(&input), 168);
    }
}
