// https://adventofcode.com/2024/day/15

use std::vec;

use super::utils::get_lines;

#[derive(Debug)]
enum WarehouseEntry {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    warehouse: Vec<Vec<WarehouseEntry>>,
    moves: Vec<Move>,
}

fn parse_warehouse(warehouse_part: &[&str]) -> Vec<Vec<WarehouseEntry>> {
    warehouse_part
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => WarehouseEntry::Empty,
                    '#' => WarehouseEntry::Wall,
                    'O' => WarehouseEntry::Box,
                    '@' => WarehouseEntry::Robot,
                    _ => panic!("Unknown warehouse entry"),
                })
                .collect()
        })
        .collect()
}

fn parse_moves(moves_part: &[&str]) -> Vec<Move> {
    moves_part
        .iter()
        .filter_map(|line| {
            match line.trim() {
                    "^" => Some(Move::Up),
                    "v" => Some(Move::Down),
                    "<" => Some(Move::Left),
                    _ => None,
                }
        })
        .collect()
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let parts: Vec<_> = lines.split(|line| line.trim().is_empty()).collect();
    let warehouse_part: Vec<&str> = parts.first()
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or(vec![]);
    let moves_part: Vec<&str> = parts
        .get(1)
        .map(|v| v.iter().map(|s| s.as_str()).collect())
        .unwrap_or(vec![]);

    let warehouse = parse_warehouse(&warehouse_part);

    let moves = parse_moves(&moves_part);

    Input { warehouse, moves }
}

fn get_sum_gps(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    println!("{:?}", input);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_gps_test01() {
        assert_eq!(11, get_sum_gps("input/day15_test01.txt"));
    }

    #[test]
    fn test_get_sum_gps_test02() {
        assert_eq!(11, get_sum_gps("input/day15_test02.txt"));
    }

    #[test]
    fn test_get_sum_gps() {
        assert_eq!(0, get_sum_gps("input/day15.txt"));
    }
}
