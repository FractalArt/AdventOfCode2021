//! # Advent of Code 2021 - Day 7
//!
//! This module contains the solution of the [seventh day's challenges](https://adventofcode.com/2021/day/7).

fn move_all_to_pos(input: &[isize], pos: isize) -> usize {
    input.iter().map(|x| (x - pos).abs()).sum::<isize>() as usize
}

pub fn day_7_1(data: &[String]) -> usize {
    let input: Vec<isize> = data[0].split(',')
        .map(|c| c.parse::<isize>().unwrap())
        .collect();

    input.iter()
        .map(|&pos| move_all_to_pos(&input, pos))
        .min()
        .unwrap() as usize
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_7_1() {
        let input = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        assert_eq!(day_7_1(&input), 37);
    }



}