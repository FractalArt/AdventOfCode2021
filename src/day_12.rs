//! # Advent of Code 2021 - Day 12
//!
//! This module contains the solution of the [twelveth day's challenges](https://adventofcode.com/2021/day/12).

use std::collections::HashSet;

/// The possible nodes that can appear in connections.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Node {
    Start,
    End,
    Intern(char)
}

/// Extract the possible connections from the input
fn get_rules(data: &[String]) -> Vec<HashSet<Node>> {
    data.iter()
    .map(|line| match line.split_once("-").unwrap() {
        ("start", c) => [Node::Start, Node::Intern(c.chars().next().unwrap())].into_iter().collect::<HashSet<Node>>(),
        (c, "end") => [Node::Intern(c.chars().next().unwrap()), Node::End].into_iter().collect::<HashSet<Node>>(),
        (c1, c2) => [Node::Intern(c1.chars().next().unwrap()), Node::Intern(c2.chars().next().unwrap())].into_iter().collect::<HashSet<Node>>()
    }).collect()
}


/// Get the next possible nodes.
fn get_next_nodes(current_node: &Node, connections: &Vec<HashSet<Node>>) -> Vec<Node> {
    connections.iter()
    .filter(|conn| conn.contains(current_node))
    .map(|conn| conn.iter().find(|&c| c != current_node).unwrap().clone())
    .collect()
}


/// Given the `current_node`, count the paths from there that reach the end.
fn paths_that_reach_end(current_node: Node, connections: &Vec<HashSet<Node>>, lowercases_visited: HashSet<char>) -> usize {
    // Loop over all next nodes and check if they reach the end, sum
    let mut sum = 0;
    let possible_next_nodes = get_next_nodes(&current_node, connections);
    for node in possible_next_nodes {
        match node {
            Node::End => { sum += 1; },
            Node::Intern(c) if c.is_uppercase() => { sum += paths_that_reach_end(node, connections, lowercases_visited.clone()); },
            Node::Intern(c) => {
                if lowercases_visited.contains(&c) {
                    sum += 0
                } else {
                    let mut lowercases_visited = lowercases_visited.clone();
                    lowercases_visited.insert(c);
                    sum += paths_that_reach_end(node, connections, lowercases_visited);
                }
            }
            _ => {}
        }
    }

    sum
}

/// Find the number of paths between starting and ending points visiting each lowercase cave only once.
pub fn day_12_1(data: &[String]) -> usize {
    let connections = get_rules(data);
    paths_that_reach_end(Node::Start, &connections, HashSet::new())
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
    }
}
