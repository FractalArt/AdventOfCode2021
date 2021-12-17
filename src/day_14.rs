//! # Advent of Code 2021 - Day 14
//!
//! This module contains the solution of the [fourteenth day's challenges](https://adventofcode.com/2021/day/14).
use itertools::Itertools;
use std::collections::HashMap;

type LetterCounter = HashMap<char, usize>;
type ConnectionCounter = HashMap<(char, char), usize>;
type Connections = HashMap<(char, char), char>;

/// Parse the input.
fn parse_input(data: &[String]) -> (LetterCounter, ConnectionCounter, Connections) {
    (
        data[0].chars().fold(HashMap::new(), |mut hm, c| {
            *hm.entry(c).or_insert(0) += 1;
            hm
        }),
        data[0]
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut hm, (c1, c2)| {
                *hm.entry((c1, c2)).or_insert(0) += 1;
                hm
            }),
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

/// Perform a single iteration of pair insertions.
fn perform_single_pair_insertion_iteration(
    lc: &mut LetterCounter,
    cc: &mut ConnectionCounter,
    conns: &Connections,
) {
    let cc_clone = cc.clone();

    conns.iter().for_each(|(&conn, &insert)| {
        let n_conns = if let Some(&n_conns) = cc_clone.get(&conn) {
            n_conns
        } else {
            0
        };
        if n_conns > 0 {
            *cc.entry((conn.0, insert)).or_insert(0) += n_conns;
            *cc.entry((insert, conn.1)).or_insert(0) += n_conns;
            *cc.get_mut(&conn).unwrap() -= n_conns;
            *lc.entry(insert).or_insert(0) += n_conns;
        }
    });
}

/// Compute the difference between the counts of the elements appearing the most and the least.
pub fn day_14(data: &[String], steps: usize) -> usize {
    let (mut lc, mut cc, conns) = parse_input(data);
    for _ in 0..steps {
        perform_single_pair_insertion_iteration(&mut lc, &mut cc, &conns);
    }

    let (min, max) = lc.values().minmax().into_option().unwrap();
    max - min
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> (LetterCounter, ConnectionCounter, Connections) {
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

        parse_input(&data)
    }

    #[test]
    fn test_parse_input() {
        let (lc, cc, conns) = get_test_input();
        assert_eq!(lc[&'N'], 2);
        assert_eq!(lc[&'C'], 1);
        assert_eq!(lc[&'B'], 1);

        assert_eq!(
            conns,
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
        );

        assert_eq!(cc[&('N', 'N')], 1);
        assert_eq!(cc[&('N', 'C')], 1);
        assert_eq!(cc[&('C', 'B')], 1);
        assert!(cc.get(&('A', 'A')).is_none());
    }

    #[test]
    fn test_perform_single_pair_insertion_iteration() {
        let (mut lc, mut cc, conns) = get_test_input();

        perform_single_pair_insertion_iteration(&mut lc, &mut cc, &conns);

        assert_eq!(lc[&'N'], 2);
        assert_eq!(lc[&'C'], 2);
        assert_eq!(lc[&'B'], 2);
        assert_eq!(lc[&'H'], 1);

        // Content at start: NNCB
        // Content after 1 iteration: NCNBCHB
        assert_eq!(cc[&('N', 'N')], 0);
        assert_eq!(cc[&('C', 'B')], 0);
        assert_eq!(cc[&('N', 'C')], 1);
        assert_eq!(cc[&('C', 'N')], 1);
        assert_eq!(cc[&('N', 'B')], 1);
        assert_eq!(cc[&('B', 'C')], 1);
        assert_eq!(cc[&('C', 'H')], 1);
        assert_eq!(cc[&('H', 'B')], 1);
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
        assert_eq!(day_14(&data, 10), 1588);
    }

    #[test]
    fn test_day_14_2() {
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
        assert_eq!(day_14(&data, 40), 2188189693529);
    }
}
