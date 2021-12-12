//! # Advent of Code 2021 - Day 11
//!
//! This module contains the solution of the [eleventh day's challenges](https://adventofcode.com/2021/day/11).
use itertools::Itertools;
use ndarray::{Array, Array2};

fn flash(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
    mut array: &mut Array2<u32>,
    mut tracker: &mut Array2<usize>,
) {
    if tracker[[row, col]] == 0 {
        // Make the element flash
        array[[row, col]] += 1;
        tracker[[row, col]] = 1;

        // Increase the neighbors of the flashing by one
        (row as isize - 1..=row as isize + 1)
            .cartesian_product(col as isize - 1..=col as isize + 1)
            .filter(|(r, c)| {
                *r >= 0
                    && *c >= 0
                    && *r < rows as isize
                    && *c < cols as isize
                    && !(*r == row as isize && *c == col as isize)
            })
            .for_each(|(r, c)| {
                array[[r as usize, c as usize]] += 1;
                if array[[r as usize, c as usize]] > 9 {
                    flash(r as usize, c as usize, rows, cols, &mut array, &mut tracker);
                }
            })
    }
}

/// Count the number of flashes occuring in `steps` steps.
pub fn day_11_1(data: &[String], steps: usize) -> usize {
    // Parse the input data into an array
    let mut array = Array::from_iter(
        data.iter()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .into_shape((10, 10))
    .unwrap();

    let mut flashes: usize = 0;

    for _ in 0..steps {
        // First increase energy level of each octopus
        array += 1;

        // Track the elements have already flashed
        let mut track_flashes_in_step = Array2::from_elem((10, 10), 0);

        // Now loop over all elements
        for (row, col) in (0..10).cartesian_product(0..10) {
            if array[[row, col]] > 9 {
                flash(
                    row,
                    col,
                    10,
                    10,
                    &mut array,
                    &mut &mut track_flashes_in_step,
                )
            }
        }

        flashes += track_flashes_in_step.iter().sum::<usize>();

        // Set the ones that flashed to 0

        for (row, col) in (0..10).cartesian_product(0..10) {
            if track_flashes_in_step[[row, col]] != 0 {
                array[[row, col]] = 0;
            }
        }
    }

    flashes
}

/// Count the step at which all octopuses first flash together.
pub fn day_11_2(data: &[String]) -> usize {
    // Parse the input data into an array
    let mut array = Array::from_iter(
        data.iter()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .into_shape((10, 10))
    .unwrap();

    for step in 1.. {
        // First increase energy level of each octopus
        array += 1;

        // Track the elements have already flashed
        let mut track_flashes_in_step = Array2::from_elem((10, 10), 0);

        // Now loop over all elements
        for (row, col) in (0..10).cartesian_product(0..10) {
            if array[[row, col]] > 9 {
                flash(
                    row,
                    col,
                    10,
                    10,
                    &mut array,
                    &mut &mut track_flashes_in_step,
                )
            }
        }

        // Set the ones that flashed to 0
        for (row, col) in (0..10).cartesian_product(0..10) {
            if track_flashes_in_step[[row, col]] != 0 {
                array[[row, col]] = 0;
            }
        }

        if array.iter().all(|&x| x == 0) {
            return step;
        }
    }

    panic!("At no time did all the octopuses flash together.");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_11_1() {
        let input = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];

        assert_eq!(day_11_1(&input, 2), 35);
        assert_eq!(day_11_1(&input, 100), 1656);
    }

    #[test]
    fn test_day_11_2() {
        let input = vec![
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];

        assert_eq!(day_11_2(&input), 195);
    }
}
