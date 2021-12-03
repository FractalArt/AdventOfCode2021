//! # Advent of Code 2021 - Day 3
//!
//! This module contains the solution of the [third day's challenges](https://adventofcode.com/2021/day/3).
use std::collections::HashMap;

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

// Get the index of the only remaining allowed element in the hashmap
fn get_hashmap_index(map: &HashMap<usize, bool>) -> usize {
    map.iter()
        .skip_while(|(_, &b)| !b)
        .map(|(&i, _)| i)
        .next()
        .unwrap()
}

// Get integer from array of bites
fn get_int_from_bit_array(bitvec: &[usize]) -> usize {
    bitvec
        .iter()
        .rev()
        .enumerate()
        .map(|(pow, bit)| bit * 2usize.pow(pow as u32))
        .sum()
}

// Get the number of available numbers and the sum of the contents of their `n`th bit
fn get_available_and_sum(
    numbers: &Vec<Vec<usize>>,
    map: &HashMap<usize, bool>,
    n: usize,
) -> (usize, usize) {
    map.iter()
        .filter(|(_, &flag)| flag)
        .enumerate()
        .map(|(index, (&i, _))| (index, numbers[i][n]))
        .fold((0, 0), |(_, sum), (index, n)| (index + 1, sum + n))
}

/// Solution of day 3 part 2.
pub fn day_3_2(data: &[String]) -> usize {
    // First parse the data
    let numbers: Vec<Vec<usize>> = data
        .iter()
        .map(|s| s.chars().map(|c| if c == '0' { 0 } else { 1 }).collect())
        .collect();

    let mut final_o2: Option<usize> = None;
    let mut final_co2: Option<usize> = None;

    let mut available_o2_map = (0..numbers.len())
        .map(|i| (i, true))
        .collect::<HashMap<_, _>>();
    let mut available_co2_map = (0..numbers.len())
        .map(|i| (i, true))
        .collect::<HashMap<_, _>>();

    // For each bit check which bit appears more often and store the indices
    let mut bit = 0;
    while final_o2.is_none() || final_co2.is_none() {
        // Only do this if o2 was not found yet
        if final_o2.is_none() {
            // Count sum of bits in the available numbers
            let (available, sum) = get_available_and_sum(&numbers, &available_o2_map, bit);
            let avail_o2 = std::cmp::max(sum, available - sum);

            // 1 is dominant, remove all with 0 in that bit from available o2
            if 2 * sum >= available {
                numbers.iter().enumerate().for_each(|(i, v)| {
                    if v[bit] != 1 {
                        *available_o2_map.get_mut(&i).unwrap() = false;
                    }
                });
            } else {
                numbers.iter().enumerate().for_each(|(i, v)| {
                    if v[bit] != 0 {
                        *available_o2_map.get_mut(&i).unwrap() = false;
                    }
                });
            }

            if avail_o2 == 1 {
                let index = get_hashmap_index(&available_o2_map);
                final_o2 = Some(get_int_from_bit_array(&numbers[index]));
            }
        }

        // Only do this if co2 was not found yet
        if final_co2.is_none() {
            // Count sum of bits in the available numbers
            let (available, sum): (usize, usize) =
                get_available_and_sum(&numbers, &available_co2_map, bit);
            let avail_co2 = std::cmp::min(sum, available - sum);

            // 1 is dominant, remove all with 1 in that bit from available co2
            if 2 * sum >= available {
                numbers.iter().enumerate().for_each(|(i, v)| {
                    if v[bit] != 0 {
                        *available_co2_map.get_mut(&i).unwrap() = false;
                    }
                });
            } else {
                numbers.iter().enumerate().for_each(|(i, v)| {
                    if v[bit] != 1 {
                        *available_co2_map.get_mut(&i).unwrap() = false;
                    }
                });
            }

            if avail_co2 == 1 {
                let index = get_hashmap_index(&available_co2_map);
                final_co2 = Some(get_int_from_bit_array(&numbers[index]));
            }
        }

        bit += 1;
    }

    final_co2.unwrap() * final_o2.unwrap()
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

    #[test]
    fn test_day_3_2() {
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
        assert_eq!(day_3_2(&input), 230);
    }
}
