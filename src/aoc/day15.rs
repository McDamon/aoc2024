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
    let (robot_x, robot_y) = robot_pos;
    let warehouse_cols = warehouse.len();
    let warehouse_rows = warehouse[0].len();
    match your_move {
        Move::Up => {
            for new_y in *robot_y..0 {
                println!("Up. New y: {}", new_y);
                match warehouse[*robot_x][new_y] {
                    WarehouseEntry::Empty => {
                        warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                        warehouse[*robot_x][new_y] = WarehouseEntry::Robot;
                        *robot_y = new_y;
                        break;
                    },
                    WarehouseEntry::Wall => {
                        break;
                    },
                    WarehouseEntry::Box => {
                        match warehouse[*robot_x][new_y - 1] {
                            WarehouseEntry::Empty => {
                                warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                                warehouse[*robot_x][new_y] = WarehouseEntry::Robot;
                                warehouse[*robot_x][new_y - 1] = WarehouseEntry::Box;
                                *robot_y = new_y;
                                break;
                            },
                            _ => {},
                        }
                    },
                    _ => {
                        continue
                    },
                }
            }
        },
        Move::Down => {
            for new_y in *robot_y..warehouse_rows {
                println!("Down. New y: {}", new_y);
                match warehouse[*robot_x][new_y] {
                    WarehouseEntry::Empty => {
                        warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                        warehouse[*robot_x][new_y] = WarehouseEntry::Robot;
                        *robot_y = new_y;
                        break;
                    },
                    WarehouseEntry::Wall => {
                        break;
                    },
                    WarehouseEntry::Box => {
                        match warehouse[*robot_x][new_y + 1] {
                            WarehouseEntry::Empty => {
                                warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                                warehouse[*robot_x][new_y] = WarehouseEntry::Robot;
                                warehouse[*robot_x][new_y + 1] = WarehouseEntry::Box;
                                *robot_y = new_y;
                                break;
                            },
                            _ => {},
                        }
                    },
                    _ => {
                        continue
                    },
                }
            }
        },
        Move::Left => {
            for new_x in *robot_x..0 {
                println!("Left. New x: {}", new_x);
                match warehouse[new_x][*robot_y] {
                    WarehouseEntry::Empty => {
                        warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                        warehouse[new_x][*robot_y] = WarehouseEntry::Robot;
                        *robot_x = new_x;
                    },
                    WarehouseEntry::Wall => {
                        break;
                    },
                    WarehouseEntry::Box => {
                        match warehouse[new_x - 1][*robot_y] {
                            WarehouseEntry::Empty => {
                                warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                                warehouse[new_x][*robot_y] = WarehouseEntry::Robot;
                                warehouse[new_x - 1][*robot_y] = WarehouseEntry::Box;
                                *robot_x = new_x;
                            },
                            _ => {},
                        }
                    },
                    _ => {
                        continue
                    },
                }
            }
        },
        Move::Right => {
            for new_x in *robot_x..warehouse_cols {
                println!("Right. New x: {}", new_x);
                match warehouse[new_x][*robot_y] {
                    WarehouseEntry::Empty => {
                        warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                        warehouse[new_x][*robot_y] = WarehouseEntry::Robot;
                        *robot_x = new_x;
                    },
                    WarehouseEntry::Wall => {
                        break;
                    },
                    WarehouseEntry::Box => {
                        match warehouse[new_x - 1][*robot_y] {
                            WarehouseEntry::Empty => {
                                warehouse[*robot_x][*robot_y] = WarehouseEntry::Empty;
                                warehouse[new_x][*robot_y] = WarehouseEntry::Robot;
                                warehouse[new_x - 1][*robot_y] = WarehouseEntry::Box;
                                *robot_x = new_x;
                            },
                            _ => {},
                        }
                    },
                    _ => {
                        continue
                    },
                }
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
        assert_eq!(0, get_sum_gps("input/day15_test03.txt"));
    }

    #[test]
    fn test_get_sum_gps_test04() {
        // Right
        assert_eq!(0, get_sum_gps("input/day15_test04.txt"));
    }

    #[test]
    fn test_get_sum_gps_test05() {
        // Down
        assert_eq!(0, get_sum_gps("input/day15_test05.txt"));
    }

    #[test]
    fn test_get_sum_gps_test06() {
        // Left
        assert_eq!(0, get_sum_gps("input/day15_test06.txt"));
    }

    #[test]
    fn test_get_sum_gps_test07() {
        // Up
        assert_eq!(0, get_sum_gps("input/day15_test07.txt"));
    }

    #[test]
    fn test_get_sum_gps() {
        assert_eq!(0, get_sum_gps("input/day15.txt"));
    }
}
