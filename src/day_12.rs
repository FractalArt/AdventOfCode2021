//! # Advent of Code 2021 - Day 12
//!
//! This module contains the solution of the [twelveth day's challenges](https://adventofcode.com/2021/day/12).
use std::collections::HashSet;

/// Extract the possible connections from the input
fn get_rules(data: &[String]) -> Vec<HashSet<&str>> {
    data.iter()
        .map(|line| line.split('-').collect::<HashSet<&str>>())
        .collect()
}

/// Get the next possible nodes.
fn get_next_nodes<'a>(current_node: &str, connections: &[HashSet<&'a str>]) -> Vec<&'a str> {
    connections
        .iter()
        .filter(|conn| conn.contains(current_node))
        .map(|conn| *conn.iter().find(|&&c| c != current_node).unwrap())
        .filter(|&node| node != "start")
        .collect()
}

/// Given the `current_node`, count the paths from there that reach the end.
fn paths_that_reach_end_lower_once(
    current_node: &str,
    connections: &[HashSet<&str>],
    lowercases_visited: HashSet<&str>,
) -> usize {
    // Loop over all next nodes and check if they reach the end, sum
    get_next_nodes(current_node, connections)
        .into_iter()
        .map(|node| match node {
            "end" => 1,
            s if s.chars().all(char::is_uppercase) => {
                paths_that_reach_end_lower_once(node, connections, lowercases_visited.clone())
            }
            s if !lowercases_visited.contains(s) => {
                let mut lowercases_visited = lowercases_visited.clone();
                lowercases_visited.insert(s);
                paths_that_reach_end_lower_once(node, connections, lowercases_visited)
            }
            _ => 0,
        })
        .sum()
}

/// Given the `current_node`, count the paths from there that reach the end.
fn paths_that_reach_end_lower_single_max_twice(
    current_node: &str,
    connections: &[HashSet<&str>],
    lowercases_visited: HashSet<&str>,
    lower_case_appears_twice: bool,
) -> usize {
    // Loop over all next nodes and check if they reach the end, sum
    get_next_nodes(current_node, connections)
        .into_iter()
        .map(|node| match node {
            "end" => 1,
            s if s.chars().all(char::is_uppercase) => paths_that_reach_end_lower_single_max_twice(
                node,
                connections,
                lowercases_visited.clone(),
                lower_case_appears_twice,
            ),
            s if !lowercases_visited.contains(s) => {
                let mut lowercases_visited = lowercases_visited.clone();
                lowercases_visited.insert(s);
                paths_that_reach_end_lower_single_max_twice(
                    node,
                    connections,
                    lowercases_visited,
                    lower_case_appears_twice,
                )
            }
            s if lowercases_visited.contains(s) && !lower_case_appears_twice => {
                paths_that_reach_end_lower_single_max_twice(
                    node,
                    connections,
                    lowercases_visited.clone(),
                    true,
                )
            }
            _ => 0,
        })
        .sum()
}

/// Find the number of paths between starting and ending points visiting each lowercase cave only once.
pub fn day_12_1(data: &[String]) -> usize {
    paths_that_reach_end_lower_once("start", &get_rules(data), HashSet::new())
}

/// Find the number of paths between starting and ending points.
///
/// Each lowercase cave may only be visited once, except for a single one which may be visited twice.
pub fn day_12_2(data: &[String]) -> usize {
    paths_that_reach_end_lower_single_max_twice("start", &get_rules(data), HashSet::new(), false)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_12_1() {
        let data = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];

        assert_eq!(day_12_1(&data), 10);

        let data = vec![
            "fs-end".to_string(),
            "he-DX".to_string(),
            "fs-he".to_string(),
            "start-DX".to_string(),
            "pj-DX".to_string(),
            "end-zg".to_string(),
            "zg-sl".to_string(),
            "zg-pj".to_string(),
            "pj-he".to_string(),
            "RW-he".to_string(),
            "fs-DX".to_string(),
            "pj-RW".to_string(),
            "zg-RW".to_string(),
            "start-pj".to_string(),
            "he-WI".to_string(),
            "zg-he".to_string(),
            "pj-fs".to_string(),
            "start-RW".to_string(),
        ];

        assert_eq!(day_12_1(&data), 226);
    }

    #[test]
    fn test_day_12_2() {
        let data = vec![
            "start-A".to_string(),
            "start-b".to_string(),
            "A-c".to_string(),
            "A-b".to_string(),
            "b-d".to_string(),
            "A-end".to_string(),
            "b-end".to_string(),
        ];

        assert_eq!(day_12_2(&data), 36);

        let data = vec![
            "fs-end".to_string(),
            "he-DX".to_string(),
            "fs-he".to_string(),
            "start-DX".to_string(),
            "pj-DX".to_string(),
            "end-zg".to_string(),
            "zg-sl".to_string(),
            "zg-pj".to_string(),
            "pj-he".to_string(),
            "RW-he".to_string(),
            "fs-DX".to_string(),
            "pj-RW".to_string(),
            "zg-RW".to_string(),
            "start-pj".to_string(),
            "he-WI".to_string(),
            "zg-he".to_string(),
            "pj-fs".to_string(),
            "start-RW".to_string(),
        ];

        assert_eq!(day_12_2(&data), 3509);
    }
}
