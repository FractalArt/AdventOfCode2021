//! # Advent of Code 2021 - Day 8
//!
//! This module contains the solution of the [eigth day's challenges](https://adventofcode.com/2021/day/8).
use std::collections::{HashMap, HashSet};

/// Count the number of times the digits `1`, `4`, `7` or `8` appear in the output.
///
/// They can be easily spotted as they have a unique number of segments, namely 2 for `1`,
/// 4 for `4`, 3 for `7` and 7 for `8`.
pub fn day_8_1(data: &[String]) -> usize {
    data.iter()
        .map(|s| s.split('|').last().unwrap().trim())
        .map(|out| {
            out.split_whitespace()
                .filter(|number| matches!(number.chars().count(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum()
}

/// Determine the sum of all the numbers represented in the output.
///
/// To do so, we start by adding an index to each segment. This is done according to
///
/// ```sh
/// 11111
/// 6   2
/// 6   2
/// 77777
/// 5   3
/// 5   3
/// 44444
/// ```
///
/// Then the algorithm proceeds in 6 steps.
///
/// 1. Identify the signals corresponding to the numbers 1, 4, 7 and 8.
///    This is easy as they have a unique number of segments.
/// 2. The difference of the signals for numbers 1 and 7 gives the letter associated
///    to segment 1.
/// 3. Take the signal corresponding to number 8 and compare it to the 3 numbers with
///    6 segmets.
/// 4. Find the one differing char that appears in the signal of 8 but not 1. This
///    gives segment 2.
/// 5. Segment 3 is given by the other char appearing the signal of number 1.
/// 6. Find the differing char that appear in the signal of number 4.
///    This gives segment 7.
/// 7. The remaining char in the signal of number 4 gives segment 6.
/// 8. The last char in the differences with the signal of eight gives segement 5,
/// 9. The only letter that has not been assigned belongs to segment 4.
pub fn identify_output(data: Vec<HashSet<char>>, out: Vec<String>) -> usize {
    // Create a hashmap mapping each letter to a segment index
    let mut map: HashMap<char, Option<usize>> = "abcdefg".chars().map(|c| (c, None)).collect();
    // Create a map that maps specific signals to integers
    let digit_map: HashMap<Vec<usize>, usize> = HashMap::from([
        (vec![1, 2, 3, 4, 5, 6], 0),
        (vec![2, 3], 1),
        (vec![1, 2, 4, 5, 7], 2),
        (vec![1, 2, 3, 4, 7], 3),
        (vec![2, 3, 6, 7], 4),
        (vec![1, 3, 4, 6, 7], 5),
        (vec![1, 3, 4, 5, 6, 7], 6),
        (vec![1, 2, 3], 7),
        (vec![1, 2, 3, 4, 5, 6, 7], 8),
        (vec![1, 2, 3, 4, 6, 7], 9),
    ]);

    // Identify the HashSets corresponding to the numbers 1, 4, 7, 8
    let one = data.iter().find(|&h| h.len() == 2).unwrap();
    let four = data.iter().find(|&h| h.len() == 4).unwrap();
    let seven = data.iter().find(|&h| h.len() == 3).unwrap();
    let eight = data.iter().find(|&h| h.len() == 7).unwrap();

    // Keep only the elements of four that do not appear in one
    let four: HashSet<_> = four.difference(one).collect();

    let segment_1_letter = seven.difference(one).into_iter().next().unwrap();
    *map.get_mut(segment_1_letter).unwrap() = Some(1);

    // Get the 3 6-segment numbers
    let six_segment_numbers: Vec<_> = data.iter().filter(|h| h.len() == 6).collect();
    let six_segment_diff_8: Vec<_> = six_segment_numbers
        .iter()
        .map(|&six| eight.difference(six).into_iter().next().unwrap())
        .collect();

    // Find the segment 2 letter
    let segment_2_letter = six_segment_diff_8
        .iter()
        .find(|&&c| one.contains(c))
        .unwrap();
    *map.get_mut(segment_2_letter).unwrap() = Some(2);

    let segment_3_letter = one.iter().find(|&c| &c != segment_2_letter).unwrap();
    *map.get_mut(segment_3_letter).unwrap() = Some(3);

    // Find the segment 4 letter
    let segment_7_letter = six_segment_diff_8
        .iter()
        .find(|&&c| four.contains(c))
        .unwrap();
    *map.get_mut(segment_7_letter).unwrap() = Some(7);

    // Other remaining char in signal of four gives segment 6
    let segment_6_letter = four.into_iter().find(|&c| &c != segment_7_letter).unwrap();
    *map.get_mut(segment_6_letter).unwrap() = Some(6);

    // Remaining diff gives segment 5
    let segment_5_letter = six_segment_diff_8
        .iter()
        .find(|&&c| &c != segment_2_letter && &c != segment_7_letter)
        .unwrap();
    *map.get_mut(segment_5_letter).unwrap() = Some(5);

    // Remainng letter is segment 4
    let key = map
        .iter()
        .find_map(|(k, v)| if v.is_none() { Some(*k) } else { None })
        .unwrap();
    *map.get_mut(&key).unwrap() = Some(4);

    out.into_iter()
        .map(|o| {
            o.chars()
                .map(|letter| map[&letter].unwrap())
                .collect::<Vec<_>>()
        })
        .map(|mut v| {
            v.sort_unstable();
            v
        })
        .map(|vec| digit_map[&vec])
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * 10usize.pow(i as u32))
}

/// Compute the sum of the output numbers
pub fn day_8_2(data: &[String]) -> usize {
    data.iter()
        .map(|s| s.split('|'))
        .map(|mut i| {
            let input = i
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.trim().chars().collect())
                .collect();
            let output = i
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            identify_output(input, output)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_8_1() {
        let input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        assert_eq!(day_8_1(&input), 26);
    }

    #[test]
    fn test_identify_output() {
        let data: Vec<HashSet<_>> = vec![
            "acedgfb".chars().collect(),
            "cdfbe".chars().collect(),
            "gcdfa".chars().collect(),
            "fbcad".chars().collect(),
            "dab".chars().collect(),
            "cefabd".chars().collect(),
            "cdfgeb".chars().collect(),
            "eafb".chars().collect(),
            "cagedb".chars().collect(),
            "ab".chars().collect(),
        ];

        let out = vec![
            "cdfeb".to_string(),
            "fcadb".to_string(),
            "cdfeb".to_string(),
            "cdbaf".to_string(),
        ];

        assert_eq!(identify_output(data, out), 5353);
    }

    #[test]
    fn test_day_8_2() {
        let input = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ];

        assert_eq!(day_8_2(&input), 61229);
    }
}
