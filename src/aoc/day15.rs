// https://adventofcode.com/2024/day/15

use core::panic;
use std::vec;

use super::utils::get_lines;

#[derive(Debug, Clone, PartialEq)]
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
    let mut moves: Vec<Move> = vec![];
    for move_line in moves_part {
        move_line.chars().for_each(|c| {
            match c {
                '^' => moves.push(Move::Up),
                'v' => moves.push(Move::Down),
                '<' => moves.push(Move::Left),
                '>' => moves.push(Move::Right),
                _ => panic!("Unknown move"),
            };
        });
    }
    moves
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let parts: Vec<_> = lines.split(|line| line.trim().is_empty()).collect();
    let warehouse_part: Vec<&str> = parts
        .first()
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

fn print_warehouse(warehouse: &[Vec<WarehouseEntry>]) {
    for row in warehouse {
        for entry in row {
            match entry {
                WarehouseEntry::Empty => print!("."),
                WarehouseEntry::Wall => print!("#"),
                WarehouseEntry::Box => print!("O"),
                WarehouseEntry::Robot => print!("@"),
            }
        }
        println!();
    }
}

fn get_next_move(current_pos: (usize, usize), your_move: &Move) -> (usize, usize) {
    let (current_x, current_y) = current_pos;
    match your_move {
        Move::Up => (current_x, current_y - 1),
        Move::Down => (current_x, current_y + 1),
        Move::Left => (current_x - 1, current_y),
        Move::Right => (current_x + 1, current_y),
    }
}

fn perform_move(
    warehouse: &mut [Vec<WarehouseEntry>],
    robot_pos: &mut (usize, usize),
    your_move: &Move,
) {
    //println!("Move: {:?}", your_move);
    let (robot_x, robot_y) = robot_pos;
    let mut maybe_next_move = Some(get_next_move((*robot_x, *robot_y), your_move));
    while let Some(next_move) = maybe_next_move {
        let (next_x, next_y) = next_move;
        match warehouse[next_y][next_x] {
            WarehouseEntry::Empty => {
                warehouse[*robot_y][*robot_x] = WarehouseEntry::Empty;
                warehouse[next_y][next_x] = WarehouseEntry::Robot;
                *robot_x = next_x;
                *robot_y = next_y;
                maybe_next_move = None;
            }
            WarehouseEntry::Box => {
                let (peek_x, peek_y) = get_next_move((next_x, next_y), your_move);
                match warehouse[peek_y][peek_x] {
                    WarehouseEntry::Empty => {
                        warehouse[next_y][next_x] = WarehouseEntry::Empty;
                        warehouse[peek_y][peek_x] = WarehouseEntry::Box;
                        maybe_next_move = Some(get_next_move((*robot_x, *robot_y), your_move))
                    }
                    WarehouseEntry::Box => maybe_next_move = Some((peek_x, peek_y)),
                    _ => maybe_next_move = None,
                }
            }
            _ => {
                maybe_next_move = None;
            }
        }
    }

    //print_warehouse(warehouse);
    //println!();
}

fn get_robot_pos(warehouse: &[Vec<WarehouseEntry>]) -> (usize, usize) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == WarehouseEntry::Robot {
                return (x, y);
            }
        }
    }
    panic!("Robot not found in the warehouse");
}

fn get_sum_gps(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut warehouse = input.warehouse.clone();

    let mut robot_pos: (usize, usize) = get_robot_pos(warehouse.as_slice());

    /*println!("Initial robot pos: {:?}", robot_pos);
    println!("Initial state:");
    print_warehouse(&warehouse);
    println!();*/

    for your_move in &input.moves {
        perform_move(&mut warehouse, &mut robot_pos, your_move);
    }

    warehouse.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (j, entry)| match entry {
                WarehouseEntry::Box => acc + (100 * i as u32 + j as u32),
                _ => acc,
            })
    })
}

fn print_warehouse_wider(warehouse: &[Vec<char>]) {
    for row in warehouse {
        for entry in row {
            print!("{}", entry);
        }
        println!();
    }
}

fn get_robot_pos_wider(warehouse: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == '@' {
                return (x, y);
            }
        }
    }
    panic!("Robot not found in the warehouse");
}

fn perform_move_wider(
    warehouse: &mut [Vec<char>],
    robot_pos: &mut (usize, usize),
    your_move: &Move,
) {
}

fn get_sum_gps_wider(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut warehouse_wider: Vec<Vec<char>> = input.warehouse.iter().fold(vec![], |mut acc, row| {
        let mut new_row: Vec<char> = vec![];
        for entry in row {
            match entry {
                WarehouseEntry::Empty => {
                    new_row.push('.');
                    new_row.push('.');
                }
                WarehouseEntry::Wall => {
                    new_row.push('#');
                    new_row.push('#');
                }
                WarehouseEntry::Box => {
                    new_row.push('[');
                    new_row.push(']');
                }
                WarehouseEntry::Robot => {
                    new_row.push('@');
                    new_row.push('.');
                }
            }
        }
        acc.push(new_row);
        acc
    });

    let mut robot_pos: (usize, usize) = get_robot_pos_wider(&warehouse_wider);

    println!("Initial robot pos: {:?}", robot_pos);
    println!("Initial state:");
    print_warehouse_wider(&warehouse_wider);
    println!();

    for your_move in &input.moves {
        perform_move_wider(&mut warehouse_wider, &mut robot_pos, your_move);
    }

    warehouse_wider.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (j, entry)| match entry {
                '[' => acc + (100 * i as u32 + j as u32),
                _ => acc,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_gps_test01() {
        assert_eq!(10092, get_sum_gps("input/day15_test01.txt"));
    }

    #[test]
    fn test_get_sum_gps_test02() {
        assert_eq!(2028, get_sum_gps("input/day15_test02.txt"));
    }

    #[test]
    fn test_get_sum_gps_test03() {
        // Down
        assert_eq!(1624, get_sum_gps("input/day15_test03.txt"));
    }

    #[test]
    fn test_get_sum_gps_test04() {
        // Right
        assert_eq!(1626, get_sum_gps("input/day15_test04.txt"));
    }

    #[test]
    fn test_get_sum_gps_test05() {
        // Down
        assert_eq!(2024, get_sum_gps("input/day15_test05.txt"));
    }

    #[test]
    fn test_get_sum_gps_test06() {
        // Left
        assert_eq!(1621, get_sum_gps("input/day15_test06.txt"));
    }

    #[test]
    fn test_get_sum_gps_test07() {
        // Up
        assert_eq!(1224, get_sum_gps("input/day15_test07.txt"));
    }

    #[test]
    fn test_get_sum_gps_test08() {
        // Right
        assert_eq!(1627, get_sum_gps("input/day15_test08.txt"));
    }

    #[test]
    fn test_get_sum_gps() {
        assert_eq!(1517819, get_sum_gps("input/day15.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test01() {
        assert_eq!(0, get_sum_gps_wider("input/day15_test01.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test02() {
        assert_eq!(0, get_sum_gps_wider("input/day15_test02.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test03() {
        // Down
        assert_eq!(0, get_sum_gps_wider("input/day15_test03.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test04() {
        // Right
        assert_eq!(0, get_sum_gps_wider("input/day15_test04.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test05() {
        // Down
        assert_eq!(0, get_sum_gps_wider("input/day15_test05.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test06() {
        // Left
        assert_eq!(0, get_sum_gps_wider("input/day15_test06.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test07() {
        // Up
        assert_eq!(0, get_sum_gps_wider("input/day15_test07.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test08() {
        // Right
        assert_eq!(0, get_sum_gps_wider("input/day15_test08.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider_test09() {
        assert_eq!(9021, get_sum_gps_wider("input/day15_test09.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider() {
        assert_eq!(0, get_sum_gps_wider("input/day15.txt"));
    }
}
