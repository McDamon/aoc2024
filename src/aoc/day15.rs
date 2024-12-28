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

fn process_move(
    warehouse: &mut [Vec<WarehouseEntry>],
    robot_pos: &mut (usize, usize),
    your_move: &Move,
) {
    let (robot_x, robot_y) = robot_pos.clone();
    match your_move {
        Move::Up => {
            let new_y = robot_y - 1;
            match warehouse[new_y][robot_x] {
                WarehouseEntry::Empty => {
                    warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                    warehouse[new_y][robot_x] = WarehouseEntry::Robot;
                    robot_pos.1 = new_y;
                },
                WarehouseEntry::Wall => {
                },
                WarehouseEntry::Box => {
                    match warehouse[new_y - 1][robot_x] {
                        WarehouseEntry::Empty => {
                            warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                            warehouse[new_y][robot_x] = WarehouseEntry::Robot;
                            warehouse[new_y - 1][robot_x] = WarehouseEntry::Box;
                            robot_pos.1 = new_y;
                        },
                        _ => {},
                    }
                },
                _ => {
                },
            }
        },
        Move::Down => {
            let new_y = robot_y + 1;
            match warehouse[new_y][robot_x] {
                WarehouseEntry::Empty => {
                    warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                    warehouse[new_y][robot_x] = WarehouseEntry::Robot;
                    robot_pos.1 = new_y;
                },
                WarehouseEntry::Wall => {
                },
                WarehouseEntry::Box => {
                    match warehouse[new_y + 1][robot_x] {
                        WarehouseEntry::Empty => {
                            warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                            warehouse[new_y][robot_x] = WarehouseEntry::Robot;
                            warehouse[new_y + 1][robot_x] = WarehouseEntry::Box;
                            robot_pos.1 = new_y;
                        },
                        _ => {},
                    }
                },
                _ => {
                },
            }
        },
        Move::Left => {
            let new_x = robot_x - 1;
            match warehouse[robot_y][new_x] {
                WarehouseEntry::Empty => {
                    warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                    warehouse[robot_y][new_x] = WarehouseEntry::Robot;
                    robot_pos.0 = new_x;
                },
                WarehouseEntry::Wall => {
                },
                WarehouseEntry::Box => {
                    match warehouse[robot_y][new_x - 1] {
                        WarehouseEntry::Empty => {
                            warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                            warehouse[robot_y][new_x] = WarehouseEntry::Robot;
                            warehouse[robot_y][new_x - 1] = WarehouseEntry::Box;
                            robot_pos.0 = new_x;
                        },
                        _ => {},
                    }
                },
                _ => {
                },
            }
        },
        Move::Right => {
            let new_x = robot_x + 1;
            match warehouse[robot_y][new_x] {
                WarehouseEntry::Empty => {
                    warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                    warehouse[robot_y][new_x] = WarehouseEntry::Robot;
                    robot_pos.0 = new_x;
                },
                WarehouseEntry::Wall => {
                },
                WarehouseEntry::Box => {
                    match warehouse[robot_y][new_x + 1] {
                        WarehouseEntry::Empty => {
                            warehouse[robot_y][robot_x] = WarehouseEntry::Empty;
                            warehouse[robot_y][new_x] = WarehouseEntry::Robot;
                            warehouse[robot_y][new_x + 1] = WarehouseEntry::Box;
                            robot_pos.0 = new_x;
                        },
                        _ => {},
                    }
                },
                _ => {
                },
            }
        },
    }

    println!("Robot pos: {:?}", robot_pos);
    println!("Move: {:?}", your_move);
    print_warehouse(warehouse);
    println!();
}

fn get_sum_gps(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut warehouse = input.warehouse.clone();

    let mut robot_pos: (usize, usize) = 'outer: {
        for (y, row) in warehouse.iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                if *entry == WarehouseEntry::Robot {
                    break 'outer (x, y);
                }
            }
        }
        panic!("Robot not found in the warehouse");
    };

    println!("Initial robot pos: {:?}", robot_pos);
    println!("Initial state:");
    print_warehouse(&warehouse);
    println!();

    for your_move in &input.moves {
        process_move(&mut warehouse, &mut robot_pos, your_move);
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
        assert_eq!(0, get_sum_gps("input/day15_test05.txt"));
    }

    #[test]
    fn test_get_sum_gps_test06() {
        // Left
        assert_eq!(1621, get_sum_gps("input/day15_test06.txt"));
    }

    #[test]
    fn test_get_sum_gps_test07() {
        // Up
        assert_eq!(0, get_sum_gps("input/day15_test07.txt"));
    }

    #[test]
    fn test_get_sum_gps_test08() {
        // Right
        assert_eq!(0, get_sum_gps("input/day15_test08.txt"));
    }

    #[test]
    fn test_get_sum_gps() {
        assert_eq!(0, get_sum_gps("input/day15.txt"));
    }
}
