//! # Advent of Code 2021 - Day 2
//!
//! This module contains the solution of the [second day's challenges](https://adventofcode.com/2021/day/2).

/// Compute the product of depth and horizontal coordinates of the submarine.
///
/// The input contains the `up`, `down` and `forward` tags followed by
/// the unit value that describes the amount of movement in the given direction.
pub fn day_2_1(data: &[String]) -> u32 {
    let (horizontal, depth) = data
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|v| (v.0, v.1.parse::<u32>().unwrap()))
        .fold(
            (0, 0),
            |(horizontal, depth), (direction, x)| match direction {
                "up" => (horizontal, depth - x),
                "down" => (horizontal, depth + x),
                _ => (horizontal + x, depth),
            },
        );
    horizontal * depth
}

/// Compute the product of depth and horizontal coordinates of the submarine.
///
/// The submarine starts at position (0, 0) with an `aim` of 0. The data contains instructions
/// on how to navigate:
/// - `down X` increases the aim by `X`units
/// - "up X" decreases the aim by `X` units
/// - `forward X` increases the horizontal position by `X` units and increases the depth with
/// `X` multiplied by `aim`.
pub fn day_2_2(data: &[String]) -> u32 {
    let (horizontal, depth, _) = data
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|v| (v.0, v.1.parse::<u32>().unwrap()))
        .fold(
            (0, 0, 0),
            |(horizontal, depth, aim), (direction, x)| match direction {
                "up" => (horizontal, depth, aim - x),
                "down" => (horizontal, depth, aim + x),
                _ => (horizontal + x, depth + x * aim, aim),
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
        assert_eq!(day_2_1(&input), 150);
    }

    #[test]
    fn test_day_2_2() {
        let input = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];
        assert_eq!(day_2_2(&input), 900);
    }
}
