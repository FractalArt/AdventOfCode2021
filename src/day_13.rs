//! # Advent of Code 2021 - Day 13
//!
//! This module contains the solution of the [tirteenth day's challenges](https://adventofcode.com/2021/day/13).
use itertools::Itertools;

#[derive(Debug, PartialEq)]

/// Define the folding
enum FoldAxis {
    /// Fold around the vertical axis.
    X(isize),
    /// Fold around the horizontal axis.
    Y(isize),
}

/// Extract the dot coordinates and the fold axes from the input.
fn get_dots_and_fold_axes(data: &[String]) -> (Vec<(isize, isize)>, Vec<FoldAxis>) {
    (
        data.iter()
            .take_while(|line| line != &"")
            .map(|line| line.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
            .collect(),
        data.iter()
            .skip_while(|line| line != &"")
            .skip(1)
            .map(|line| line.rsplit_once(" ").unwrap())
            .map(|(_, x)| match x.rsplit_once("=").unwrap() {
                ("x", val) => FoldAxis::X(val.parse::<isize>().unwrap()),
                (_, val) => FoldAxis::Y(val.parse::<isize>().unwrap()),
            })
            .collect(),
    )
}

/// Fold the paper containing the `dots` along the `axis`.
fn fold(dots: Vec<(isize, isize)>, axis: &FoldAxis) -> Vec<(isize, isize)> {
    match &axis {
        FoldAxis::X(z) => dots
            .into_iter()
            .map(|(x, y)| ((x - z).abs(), y))
            .sorted()
            .dedup()
            .collect(),
        FoldAxis::Y(z) => dots
            .into_iter()
            .map(|(x, y)| (x, if y < *z { y } else { z - (y - z).abs() }))
            .sorted()
            .dedup()
            .collect(),
    }
}

/// Count the number of visible dots on the transparent paper after the first fold
pub fn day_13_1(data: &[String]) -> usize {
    let (dots, axes) = get_dots_and_fold_axes(data);
    fold(dots, &axes[0]).iter().count()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_dots_and_fold_axes() {
        let data = vec![
            "0,1".to_string(),
            "4,2".to_string(),
            "7,7".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ];

        let (dots, axes) = get_dots_and_fold_axes(&data);

        assert_eq!(dots, vec![(0, 1), (4, 2), (7, 7)]);
        assert_eq!(axes, vec![FoldAxis::Y(7), FoldAxis::X(5)]);
    }

    #[test]
    fn test_fold() {
        let data = vec![
            "6,10".to_string(),
            "0,14".to_string(),
            "9,10".to_string(),
            "0,3".to_string(),
            "10,4".to_string(),
            "4,11".to_string(),
            "6,0".to_string(),
            "6,12".to_string(),
            "4,1".to_string(),
            "0,13".to_string(),
            "10,12".to_string(),
            "3,4".to_string(),
            "3,0".to_string(),
            "8,4".to_string(),
            "1,10".to_string(),
            "2,14".to_string(),
            "8,10".to_string(),
            "9,0".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ];

        let (dots, axes) = get_dots_and_fold_axes(&data);

        let folded = fold(dots, &axes[0]);

        assert_eq!(
            folded,
            vec![
                (0, 0),
                (2, 0),
                (3, 0),
                (6, 0),
                (9, 0),
                (0, 1),
                (4, 1),
                (6, 2),
                (10, 2),
                (0, 3),
                (4, 3),
                (1, 4),
                (3, 4),
                (6, 4),
                (8, 4),
                (9, 4),
                (10, 4),
            ]
            .into_iter()
            .sorted()
            .collect::<Vec<(isize, isize)>>()
        )
    }

    #[test]
    fn test_day_13_1() {
        let data = vec![
            "6,10".to_string(),
            "0,14".to_string(),
            "9,10".to_string(),
            "0,3".to_string(),
            "10,4".to_string(),
            "4,11".to_string(),
            "6,0".to_string(),
            "6,12".to_string(),
            "4,1".to_string(),
            "0,13".to_string(),
            "10,12".to_string(),
            "3,4".to_string(),
            "3,0".to_string(),
            "8,4".to_string(),
            "1,10".to_string(),
            "2,14".to_string(),
            "8,10".to_string(),
            "9,0".to_string(),
            "".to_string(),
            "fold along y=7".to_string(),
            "fold along x=5".to_string(),
        ];

        assert_eq!(day_13_1(&data), 17);
    }
}
