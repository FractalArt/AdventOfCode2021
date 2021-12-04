//! # Advent of Code 2021 - Day 4
//!
//! This module contains the solution of the [fourth day's challenges](https://adventofcode.com/2021/day/4).

use ndarray::Array2;
use std::cell::RefCell;

#[derive(Debug)]
struct Board {
    board: Array2<usize>,
    marked: RefCell<Array2<usize>>,
}

impl Board {
    /// Construct a board from the string representation of each row.
    fn from_string_rows(string_rows: &[String]) -> Self {
        let marked = RefCell::new(Array2::from_elem((5, 5), 0));
        let mut board = Array2::from_elem((5, 5), 0);
        for (row, string_row) in string_rows.iter().enumerate() {
            string_row
                .split_whitespace()
                .enumerate()
                .map(|(col, n)| (col, n.parse::<usize>().unwrap()))
                .for_each(|(col, n)| board[[row, col]] = n);
        }
        Self { board, marked }
    }

    /// Add a new number to the board.
    ///
    /// If, after the number is added, no column or row exists whose entries all match one of the
    /// drawn numbers, `None` is returned. If at least one such row or column exists, the product of the
    /// sum of the values on the board which do not match a drawn number and the last added drawn number
    /// is returned.
    pub fn mark_number(&self, number: usize) -> Option<usize> {
        // For each number, look for hits on the board and compute the sum of board
        // entries not associated to hits
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.board[[row, col]] == number {
                    self.marked.borrow_mut()[[row, col]] = 1;
                }
                if self.marked.borrow_mut()[[row, col]] == 0 {
                    sum += self.board[[row, col]];
                }
            }
        }
        // Check if there is any row or column where all entries are true. If this is the case, return the sum
        for row in self.marked.borrow().rows() {
            if row.iter().sum::<usize>() == 5 {
                return Some(sum * number);
            }
        }

        for col in self.marked.borrow().columns() {
            if col.iter().sum::<usize>() == 5 {
                return Some(sum * number);
            }
        }

        None
    }
}

/// Extract a vector of `Board`s from the input
fn get_boards_from_input(input: &[String]) -> Vec<Board> {
    input[2..]
        .chunks(6)
        .map(|chunk| chunk.into_iter().cloned().take(5).collect::<Vec<String>>())
        .map(|v| Board::from_string_rows(&v))
        .collect()
}

/// Find the winning board.
///
/// The function takes the lines from the input and constructs the bingo boards,
/// After that, the called numbers are marked on the bingo boards, one after the other.
/// As soon as one board wins, the product of the sum of its unmarked numbers and
/// the last called number that lead to the win is returned.
pub fn day_4_1(string_rows: &[String]) -> usize {
    let boards: Vec<Board> = get_boards_from_input(string_rows);

    for called_number in string_rows[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
    {
        for board in boards.iter() {
            match board.mark_number(called_number) {
                None => continue,
                Some(result) => return result,
            }
        }
    }

    panic!("No board won!");
}

/// Find the board that wins last.
///
/// The function takes the lines from the input and constructs the bingo boards,
/// After that, the called numbers are marked on the bingo boards, one after the other.
/// As soon as one board wins, the product of the sum of its unmarked numbers and
/// the last called number that lead to the win is returned. We repeat until, we find the last board.
pub fn day_4_2(string_rows: &[String]) -> usize {
    // First collect all the boards
    let boards: Vec<Board> = get_boards_from_input(string_rows);

    let mut boards_in_game = vec![true; boards.len()];

    for called_number in string_rows[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
    {
        for (board_index, board) in boards.iter().enumerate() {
            if !boards_in_game[board_index] {
                continue;
            }
            match board.mark_number(called_number) {
                None => continue,
                Some(result) => {
                    boards_in_game[board_index] = false;
                    if boards_in_game.iter().all(|b| !b) {
                        return result;
                    }
                }
            }
        }
    }

    panic!("Not all boards won!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_number() {
        let rows = vec![
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];

        let board = Board::from_string_rows(&rows);
        assert_eq!(board.mark_number(7), None);
        assert_eq!(board.mark_number(4), None);
        assert_eq!(board.mark_number(9), None);
        assert_eq!(board.mark_number(5), None);
        assert_eq!(board.mark_number(11), None);
        assert_eq!(board.mark_number(17), None);
        assert_eq!(board.mark_number(23), None);
        assert_eq!(board.mark_number(2), None);
        assert_eq!(board.mark_number(0), None);
        assert_eq!(board.mark_number(14), None);
        assert_eq!(board.mark_number(21), None);
        assert_eq!(board.mark_number(24), Some(4512));
    }

    #[test]
    fn test_day_4_1() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            " 8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            " 6 10  3 18  5".to_string(),
            " 1 12 20 15 19".to_string(),
            "".to_string(),
            " 3 15  0  2 22".to_string(),
            " 9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];

        assert_eq!(day_4_1(&input), 4512);
    }

    #[test]
    fn test_day_4_2() {
        let input = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            " 8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            " 6 10  3 18  5".to_string(),
            " 1 12 20 15 19".to_string(),
            "".to_string(),
            " 3 15  0  2 22".to_string(),
            " 9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];

        assert_eq!(day_4_2(&input), 1924);
    }
}
