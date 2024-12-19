// https://adventofcode.com/2024/day/8


use std::collections::HashMap;

use super::utils::get_lines;

struct Input {
    map: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let map_line: Vec<char> = line.chars().collect();
        map.push(map_line);
    }

    Input { map }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    value: char,
}

fn get_unique_antinodes(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let unique_antinodes = 0;

    let node_map: HashMap<char, Vec<Node>> = input.map.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter(|(_, c)| **c != '.').map(move |(j, &value)| (value, Node { x: i, y: j, value }))
    }).fold(HashMap::new(), |mut acc, (key, node)| {
        acc.entry(key).or_insert(Vec::new()).push(node);
        acc
    });

    for (c, nodes) in node_map {
        println!("Finding antinodes for {:?}", c);
        for node in nodes.clone() {
            println!("Testing node {:?}", node);
            let new_nodes = nodes.iter().filter(|&&n| n != node).cloned().collect::<Vec<Node>>();
            println!("{:?}", new_nodes);
        }
    }

    unique_antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_antinodes_test01() {
        assert_eq!(14, get_unique_antinodes("input/day08_test01.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test02() {
        assert_eq!(2, get_unique_antinodes("input/day08_test02.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test03() {
        assert_eq!(4, get_unique_antinodes("input/day08_test03.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test04() {
        assert_eq!(3, get_unique_antinodes("input/day08_test04.txt"));
    }

    #[test]
    fn test_get_unique_antinodes() {
        assert_eq!(0, get_unique_antinodes("input/day08.txt"));
    }
}
