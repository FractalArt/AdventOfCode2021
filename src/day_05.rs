//! # Advent of Code 2021 - Day 4
//!
//! This module contains the solution of the [fifth day's challenges](https://adventofcode.com/2021/day/5).
use ndarray::Array2;
use regex::Regex;
use std::cmp::{max, min};

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d*),(\d*) -> (\d*),(\d*)$").unwrap();
}

type Line = ((usize, usize), (usize, usize));

/// Get the line coordinates from the string representation in the input line.
///
/// The string representation
///
/// ```sh
/// 1,2 -> 33,44
/// ```
///
/// will be mapped to the tuple
///
/// ```sh
/// ((1, 2),(33, 44))
/// ```
fn get_coordinates(s: &str) -> Line {
    let captures = RE.captures(s).unwrap();
    (
        (
            captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        ),
        (
            captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        ),
    )
}

/// Number of points in the cartesian plane where more than one horizontal or vertical segments pass.
/// If `include_diagonal=true` then also diagonals are included.
pub fn day_5(data: &[String], include_diagonal: bool) -> usize {
    // parse the input
    let lines: Vec<Line> = data.iter().map(|s| get_coordinates(s)).collect();
    // determine the dimensions of the grid
    let x_max = *lines
        .iter()
        .map(|((x1, _), (x2, _))| max(x1, x2))
        .max()
        .unwrap();
    let y_max = *lines
        .iter()
        .map(|((_, y1), (_, y2))| max(y1, y2))
        .max()
        .unwrap();
    // create the grid (+1, because 0 is included at the beginning)
    let mut grid = Array2::from_elem((y_max + 1, x_max + 1), 0);
    // update the grid
    for line in lines {
        let ((x1, y1), (x2, y2)) = line;
        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                grid[[y, x1]] += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                grid[[y1, x]] += 1;
            }
        } else if include_diagonal {
            let mut x = x1 as isize;
            let mut y = y1 as isize;
            let steps = (x2 as isize - x1 as isize).abs();
            let x_step = (x2 as isize - x1 as isize) / steps;
            let y_step = (y2 as isize - y1 as isize) / steps;
            for _ in 0..=steps {
                grid[[y as usize, x as usize]] += 1;
                x += x_step;
                y += y_step;
            }
        } else {
            continue;
        }
    }
    // check number of entries higher than 2
    grid.iter().filter(|count| count > &&1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_coordinates() {
        assert_eq!(get_coordinates(&"1,2 -> 3,4".to_string()), ((1, 2), (3, 4)));
    }

    #[test]
    fn test_day_5_1() {
        let input = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ];
        assert_eq!(day_5(&input, false), 5);
    }

    #[test]
    fn test_day_5_2() {
        let input = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ];
        assert_eq!(day_5(&input, true), 12);
    }
}
