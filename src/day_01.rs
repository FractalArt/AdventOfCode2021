//! # Advent of Code 2021 - Day 1
//!
//! This module contains the solution of the [first day's challenges](https://adventofcode.com/2021/day/1).

use itertools::Itertools;

/// The solution to task 1 of day 1.
///
/// By scanning the depth of the ocean outward from the submarine, find out
/// how quickly the depth increases by counting how many times the sum of the
/// `window_size` elements in a window is larger than the sum of the preceding window.
pub fn day_1(data: &[u32], window_size: usize) -> u32 {
    data.windows(window_size)
        .map(|slice| slice.iter().sum())
        .tuple_windows()
        .map(|(x, y): (u32, u32)| if y > x { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_1() {
        assert_eq!(
            7,
            day_1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 1)
        );
    }

    #[test]
    fn test_day_1_2() {
        assert_eq!(
            5,
            day_1(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263], 3)
        );
    }
}
