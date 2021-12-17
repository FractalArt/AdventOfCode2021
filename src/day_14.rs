//! # Advent of Code 2021 - Day 14
//!
//! This module contains the solution of the [fourteenth day's challenges](https://adventofcode.com/2021/day/14).
use itertools::Itertools;
use std::collections::HashMap;

type InsertionRules = HashMap<(char, char), char>;

/// Parse the input into the polymer template string and the insertion rules.
fn parse_input(data: &[String]) -> (String, InsertionRules) {
    (
        data[0].clone(),
        data.iter()
            .skip(2)
            .map(|s| s.split_once(" -> ").unwrap())
            .fold(HashMap::new(), |mut hm, (target, insert)| {
                hm.insert(
                    target.chars().tuples().next().unwrap(),
                    insert.chars().next().unwrap(),
                );
                hm
            }),
    )
}

/// Perform or single iteration of pair insertions
fn single_pair_insertion_process(template: String, rules: &InsertionRules) -> String {
    template
        .chars()
        .zip(template.chars().skip(1))
        .flat_map(|pair| {
            if let Some(c) = rules.get(&pair) {
                vec![pair.0, *c]
            } else {
                vec![pair.0]
            }
        })
        .chain(template.chars().rev().take(1))
        .join("")
}

/// Compute the difference between the most and least common elements in the polymer after `steps` iterations.
pub fn day_14_1(data: &[String], steps: usize) -> usize {
    let (mut template, rules) = parse_input(data);

    for _ in 0..steps {
        template = single_pair_insertion_process(template, &rules);
    }

    let counts: HashMap<char, _> = template.chars().fold(HashMap::new(), |mut hm, c| {
        *hm.entry(c).or_insert(0) += 1;
        hm
    });
    let (min, max) = counts.values().minmax().into_option().unwrap();

    max - min
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let data = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let (polymer_template, rules) = parse_input(&data);
        assert_eq!(polymer_template, "NNCB");
        assert_eq!(
            rules,
            [
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]
            .into_iter()
            .collect()
        )
    }

    #[test]
    fn test_single_pair_insertion_process() {
        let data = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        let (polymer_template, rules) = parse_input(&data);

        assert_eq!(
            single_pair_insertion_process(polymer_template, &rules),
            "NCNBCHB".to_string()
        );
    }

    #[test]
    fn test_day_14_1() {
        let data = vec![
            "NNCB".to_string(),
            "".to_string(),
            "CH -> B".to_string(),
            "HH -> N".to_string(),
            "CB -> H".to_string(),
            "NH -> C".to_string(),
            "HB -> C".to_string(),
            "HC -> B".to_string(),
            "HN -> C".to_string(),
            "NN -> C".to_string(),
            "BH -> H".to_string(),
            "NC -> B".to_string(),
            "NB -> B".to_string(),
            "BN -> B".to_string(),
            "BB -> N".to_string(),
            "BC -> B".to_string(),
            "CC -> N".to_string(),
            "CN -> C".to_string(),
        ];

        assert_eq!(day_14_1(&data, 10), 1588);
    }
}
