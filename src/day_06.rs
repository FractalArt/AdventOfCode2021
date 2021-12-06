//! # Advent of Code 2021 - Day 6
//!
//! This module contains the solution of the [sixth day's challenges](https://adventofcode.com/2021/day/6).

/// Compute the number of lanternfish after a given number of `days`.
///
/// To have an algorithm constant in time and memory, we only keep track of the number of animals per timer.
/// For the initial state `3,4,3,1,2` there are thus 0 animals for timer 0, 1 animal for timer 1, 1 animal for
/// timer 2, 2 animals for timer 3 and 1 animal for timer 4. For timers 5 to 8 there are no animals.
/// After each day, the number of animals per timer decreases by one, i.e. everything gets shifted cyclically
/// to the left (w.r.t. animals per timer). The number of animals with timer 0 at day `n` corresponds to the number
/// of animals at category 8 at day `n+1`. The only thing to take into consideration is that each 0 timer also yields another 6.
/// Hence the number of animals for timer 6 on day `n+1` corresponds to the number of animals for timer 7 on day `n`, plus the
/// number of animals on timer 0 on day `n`.
pub fn day_6(data: &[String], days: usize) -> usize {
    let mut animals_per_timer: Vec<usize> = data
        .iter()
        .flat_map(|v| v.split(','))
        .map(|c| c.parse::<usize>().unwrap())
        .fold(vec![0; 9], |mut acc, i| {
            acc[i] += 1;
            acc
        });

    for _ in 0..days {
        let zero = animals_per_timer[0];
        for i in 1..=8 {
            animals_per_timer[i - 1] = animals_per_timer[i];
        }
        animals_per_timer[8] = zero;
        animals_per_timer[6] += zero;
    }

    animals_per_timer.iter().sum()
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
