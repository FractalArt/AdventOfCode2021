//! # Advent of Code 2021 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2021/day/3).

/// Compute the products of the parameters `gamma` and `epsilon` from the diagnostic report.
///
/// Each bit in the `gamma` rate corresponds to the most common bit in the corresponding position
/// of all numbers in the diagnostic report (the input `data`).
/// The bits in the `epsilon` rate are computed similarly, except that the least common bit in the
/// corresponding position of the input numbers is chosen.
///
/// __NOTE__: In principle `epsilon` can be easily computed from `gamma` by using bitwise or. However,
/// this will also invert the leading 0 bits, which is why compute both of them manually.
pub fn day_3_1(data: &[String]) -> usize {
    let entries = data.len(); // Number of inputs
    let bits = data[0].len(); // Number of bits in each input

    // Flatten the input and extract the bits as integers
    let flat: Vec<isize> = data
        .iter()
        .flat_map(|s| s.chars().map(|c| if c == '0' { 0 } else { 1 }))
        .collect();

    // Compute gamma & epsilon
    let (gamma, epsilon) = (0..bits)
        // for each position, compute the most common bit
        .map(|i| flat.iter().skip(i).step_by(bits).sum())
        .map(|sum: isize| if 2 * sum as usize > entries { 1 } else { 0 })
        // convert binary representation of `gamma` to decimal representations of `gamma`and `epsilon`.
        .rev()
        .enumerate()
        .fold((0, 0), |(gamma, epsilon), (pow, bit)| {
            (
                gamma + bit * 2usize.pow(pow as u32),
                epsilon + (bit as isize - 1).abs() as usize * 2usize.pow(pow as u32),
            )
        });

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_1() {
        let input = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        assert_eq!(day_3_1(&input), 198);
    }
}
