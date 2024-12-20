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
    pos: (usize, usize),
    val: char,
}

fn print_map(map: &[Vec<char>]) {
    println!("Grid:");
    for map_row in map.iter() {
        for map_entry in map_row {
            print!("{:#}", *map_entry);
        }
        println!();
    }
}

fn get_unique_antinodes(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let node_map: HashMap<char, Vec<Node>> = input
        .map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c != '.')
                .map(move |(j, &val)| {
                    (
                        val,
                        Node {
                            pos: (i, j),
                            val: val,
                        },
                    )
                })
        })
        .fold(HashMap::new(), |mut acc, (key, node)| {
            acc.entry(key).or_insert(Vec::new()).push(node);
            acc
        });

    let mut unique_antinodes: u32 = 0;

    let mut antinode_map = input.map.clone();

    for (_c, nodes) in node_map {
        for node in nodes.clone() {
            let (row, col) = node.pos;
            let other_nodes = nodes
                .iter()
                .filter(|&&n| n != node)
                .cloned()
                .collect::<Vec<Node>>();
            for other_node in other_nodes {
                let (other_row, other_col) = other_node.pos;

                let row_diff = row as i32 - other_row as i32;
                let col_diff = col as i32 - other_col as i32;
                /*println!(
                    "Testing {:?} against other node {:?}, row_diff = {:?}, col_diff = {:?}",
                    node, other_node, row_diff, col_diff
                );*/

                let antinode_pos: Option<(usize, usize)> = if row_diff.is_negative()
                    && col_diff.is_negative()
                {
                    let new_row = row as i32 - row_diff.abs();
                    let new_col = col as i32 - col_diff.abs();
                    if new_row < 0 || new_col < 0 {
                        None
                    } else {
                        Some((new_row as usize, new_col as usize))
                    }
                } else if row_diff.is_negative() && col_diff.is_positive() {
                    let new_row = row as i32 - row_diff.abs();
                    let new_col = col as i32 + col_diff.abs();
                    if new_row < 0 || new_col >= antinode_map[0].len() as i32 {
                        None
                    } else {
                        Some((new_row as usize, new_col as usize))
                    }
                } else if row_diff.is_positive() && col_diff.is_negative() {
                    let new_row = row as i32 + row_diff.abs();
                    let new_col = col as i32 - col_diff.abs();
                    if new_row >= antinode_map.len() as i32 || new_col < 0 {
                        None
                    } else {
                        Some((new_row as usize, new_col as usize))
                    }
                } else if row_diff.is_positive() && col_diff.is_positive() {
                    let new_row = row as i32 + row_diff.abs();
                    let new_col = col as i32 + col_diff.abs();
                    if new_row >= antinode_map.len() as i32 || new_col >= antinode_map[0].len() as i32
                    {
                        None
                    } else {
                        Some((new_row as usize, new_col as usize))
                    }
                } else {
                    None
                };

                if let Some((antinode_row, antinode_col)) = antinode_pos {
                    if let Some(map_node) = antinode_map
                        .get(antinode_row)
                        .and_then(|row| row.get(antinode_col))
                    {
                        match *map_node {
                            '.' => {
                                antinode_map[antinode_row][antinode_col] = '#';
                                unique_antinodes += 1;
                            },
                            '#' => {
                                println!(
                                    "Antinode already occupied: {:?}",
                                    (antinode_row, antinode_col)
                                );
                            }
                            _ => {
                                unique_antinodes += 1;
                            }
                        }
                    } else {
                        println!("Antinode out of bounds: {:?}", (antinode_row, antinode_col));
                    };
                }
            }
        }
    }

    print_map(&antinode_map);

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
