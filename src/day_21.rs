//! # Advent of Code 2021 - Day 21
//!
//! This module contains the solution of the [twenty-first day's challenges](https://adventofcode.com/2021/day/21).
use std::collections::HashMap;

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

type State = ((usize, usize), (usize, usize));

/// Compute the number of universes in which the player winning most matches wins.
///
/// Instead of tracking all states individually in a vector (which requires a lot of memory),
/// the different states and their number of occurrences are tracked.
///
/// For each turn, of three dice throws, a player creates an additional twenty seven universes. These correspond to the
/// following sums of dice throws (e.g. forward steps for the player in that turn):
///
/// | Forard | Dice combinations            |     Number of combinations   |
/// | :----: | :---------------------:      | :--------------------------: |
/// | 3      |  111                         |  1                           |
/// | 4      |  112 121 211                 |  3                           |
/// | 5      |  113 122 131 212 221 311     |  6                           |
/// | 6      |  123 131 213 222 231 312 321 |  7                           |
/// | 7      |  133 223 232 313 322 331     |  6                           |
/// | 8      |  233 323 332                 |  3                           |
/// | 9      |  333                         |  1                           |
pub fn day_21_2(start_1: usize, start_2: usize) -> usize {
    let mut player_1_wins = 0;
    let mut player_2_wins = 0;

    // Track the different states and their counts
    let mut state_counts: HashMap<State, usize> = vec![(((start_1, 0), (start_2, 0)), 1)]
        .into_iter()
        .collect();

    let dice_possiblities: HashMap<usize, usize> =
        vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
            .into_iter()
            .collect();

    loop {
        // Player 1
        let mut tmp = HashMap::new();
        for (((old_pos_1, old_score_1), player_2), count) in state_counts {
            for (forward, possibilites) in dice_possiblities.iter() {
                let pos_1 = (old_pos_1 + forward) % 10;
                let score_1 = if pos_1 == 0 {
                    old_score_1 + 10
                } else {
                    old_score_1 + pos_1
                };
                if score_1 >= 21 {
                    player_1_wins += count * possibilites;
                } else {
                    let entry = ((pos_1, score_1), player_2);
                    *tmp.entry(entry).or_insert(0) += count * possibilites;
                }
            }
        }
        state_counts = tmp;

        // Player 2
        let mut tmp = HashMap::new();
        for ((player_1, (old_pos_2, old_score_2)), count) in state_counts {
            for (forward, possibilites) in dice_possiblities.iter() {
                let pos_2 = (old_pos_2 + forward) % 10;
                let score_2 = if pos_2 == 0 {
                    old_score_2 + 10
                } else {
                    old_score_2 + pos_2
                };
                if score_2 >= 21 {
                    player_2_wins += count * possibilites;
                } else {
                    let entry = (player_1, (pos_2, score_2));
                    *tmp.entry(entry).or_insert(0) += count * possibilites;
                }
            }
        }
        state_counts = tmp;

        if state_counts.is_empty() {
            break;
        }
    }

    std::cmp::max(player_1_wins, player_2_wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_21_1() {
        assert_eq!(day_21_1(4, 8), 739785);
    }

    #[test]
    fn test_day_21_2() {
        assert_eq!(day_21_2(4, 8), 444356092776315);
    }
}
