//! # Advent of Code 2021 - Day 9
//!
//! This module contains the solution of the [nineth day's challenges](https://adventofcode.com/2021/day/9).
use itertools::Itertools;
use ndarray::{Array, Array2};

/// Check if a point is a low point
fn is_low_point(row: isize, col: isize, rows: usize, cols: usize, array: &Array2<u32>) -> bool {
    [
        (row - 1, col),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col),
    ]
    .iter()
    .filter(|(r, c)| r >= &0 && c >= &0 && *r < rows as isize && *c < cols as isize)
    .all(|(r, c)| array[[row as usize, col as usize]] < array[[*r as usize, *c as usize]])
}

/// Compute the sum of the risk levels of the low points in the area.
///
/// The risk level of each point is given by its height plus `1`.
pub fn day_9_1(data: &[String]) -> u32 {
    let rows = data.len();
    let cols = data[0].chars().count();

    let array = Array::from_iter(
        data.iter()
            .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .into_shape((rows, cols))
    .unwrap();

    // Loop over all combinations of rows and cols
    (0..rows as isize)
        .cartesian_product(0..cols as isize)
        // for each row and col loop over adjacent and keep only those that are minimal
        .filter(|(row, col)| is_low_point(*row, *col, rows, cols, &array))
        .map(|(row, col)| array[[row as usize, col as usize]] + 1)
        .sum()
}

/// Map the basin corresponding to the local minimum `(row, col)`.
fn map_basin(
    row: isize,
    col: isize,
    rows: usize,
    cols: usize,
    array: &Array2<u32>,
    mut tracker: &mut Array2<u32>,
) {
    // Look at the adjacent positions and see if they belong to the position
    [
        (row - 1, col),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col),
    ]
    .iter()
    .filter(|(r, c)| r >= &0 && c >= &0 && *r < rows as isize && *c < cols as isize)
    .filter(|(r, c)| {
        array[[*r as usize, *c as usize]] > array[[row as usize, col as usize]]
            && array[[*r as usize, *c as usize]] != 9
    })
    .for_each(|(r, c)| {
        if tracker[[*r as usize, *c as usize]] == 0 {
            // The point in question belongs to the basin
            *tracker.get_mut([*r as usize, *c as usize]).unwrap() = 1;
            map_basin(*r, *c, rows, cols, &array, &mut tracker);
        }
    });
}

/// Determine the product of the number of elements in each one of the three largest basins.
pub fn day_9_2(data: &[String]) -> u32 {
    let rows = data.len();
    let cols = data[0].chars().count();

    // Parse the input into an array
    let array = Array::from_iter(
        data.iter()
            .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap())),
    )
    .into_shape((rows, cols))
    .unwrap();

    // Loop over all fields in the area and if they are a local minimum, determine the size of their surrounding basin
    (0..rows as isize)
        .cartesian_product(0..cols as isize)
        .filter(|(row, col)| is_low_point(*row, *col, rows, cols, &array))
        .map(|(row, col)| {
            // Create an array, tracking which fields already belong to the basin
            let mut tracker = Array2::from_elem((rows, cols), 0);
            *tracker.get_mut([row as usize, col as usize]).unwrap() = 1;
            map_basin(row, col, rows, cols, &array, &mut tracker);
            tracker.iter().sum::<u32>()
        })
        .sorted()
        .rev()
        .take(3)
        .product()
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

    #[test]
    fn test_map_basin() {
        let input = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        let rows = 5;
        let cols = 10;

        let array = Array::from_iter(
            input
                .iter()
                .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap())),
        )
        .into_shape((rows, cols))
        .unwrap();

        let row = 0;
        let col = 1;
        let mut tracker = Array2::from_elem((rows, cols), 0);
        *tracker.get_mut([row as usize, col as usize]).unwrap() = 1;
        map_basin(row, col, rows, cols, &array, &mut tracker);
        assert_eq!(tracker.iter().sum::<u32>(), 3);

        let row = 0;
        let col = 9;
        let mut tracker = Array2::from_elem((rows, cols), 0);
        *tracker.get_mut([row as usize, col as usize]).unwrap() = 1;
        map_basin(row, col, rows, cols, &array, &mut tracker);
        assert_eq!(tracker.iter().sum::<u32>(), 9);
    }

    #[test]
    fn test_day_9_2() {
        let input = vec![
            "2199943210".to_string(),
            "3987894921".to_string(),
            "9856789892".to_string(),
            "8767896789".to_string(),
            "9899965678".to_string(),
        ];

        assert_eq!(day_9_2(&input), 1134);
    }
}
