//! # Advent of Code 2021 - Day 21
//!
//! This module contains the solution of the [twenty-first day's challenges](https://adventofcode.com/2021/day/21).

/// Compute the product of dice rolls times score of the losing player.
///
/// The die always gives the number 1-100 in a row and starts over when reaching 100.
pub fn day_21_1(start_1: usize, start_2: usize) -> usize {
    let (mut pos_1, mut pos_2) = (start_1, start_2);
    let (mut score_1, mut score_2) = (0, 0);

    let mut iter = (1..=100).cycle();
    let mut dice_rolls = 0;

    loop {
        // Player one plays
        for _ in 0..3 {
            pos_1 += iter.next().unwrap();
        }

        pos_1 %= 10;
        score_1 += if pos_1 == 0 { 10 } else { pos_1 };
        dice_rolls += 3;

        if score_1 >= 1000 {
            break score_2 * dice_rolls;
        }

        // Player two plays
        for _ in 0..3 {
            pos_2 += iter.next().unwrap();
        }

        pos_2 %= 10;
        score_2 += if pos_2 == 0 { 10 } else { pos_2 };
        dice_rolls += 3;

        if score_2 >= 1000 {
            break score_1 * dice_rolls;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_21_1() {
        assert_eq!(day_21_1(4, 8), 739785);
    }
}
