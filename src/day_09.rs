//! # Advent of Code 2021 - Day 9
//!
//! This module contains the solution of the [nineth day's challenges](https://adventofcode.com/2021/day/9).
use itertools::Itertools;

/// Compute the sum of the risk levels of the low points in the area.
///
/// The risk level of each point is given by its height plus `1`.
pub fn day_9_1(data: &[String]) -> u32 {
    let rows = data.len();
    let cols = data[0].chars().count();

    let array = ndarray::Array::from_iter(
        data.iter()
            .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .into_shape((rows, cols))
    .unwrap();

    // Loop over all combinations of rows and cols
    (0..rows as isize)
        .cartesian_product(0..cols as isize)
        // for each row and col loop over adjacent and keep only those that are minimal
        .filter(|(row, col)| {
            [
                (*row - 1, *col),
                (*row, *col - 1),
                (*row, *col + 1),
                (*row + 1, *col),
            ]
            .iter()
            .filter(|(r, c)| r >= &0 && c >= &0 && *r < rows as isize && *c < cols as isize)
            .all(|(r, c)| array[[*row as usize, *col as usize]] < array[[*r as usize, *c as usize]])
        })
        .map(|(row, col)| array[[row as usize, col as usize]] + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_9_1() {
        let input = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        assert_eq!(day_9_1(&input), 15);
    }
}
