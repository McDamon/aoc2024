// https://adventofcode.com/2024/day/15

use core::panic;
use std::{
    collections::{HashSet, VecDeque},
    vec,
};

use super::utils::get_lines;

#[derive(Debug, PartialEq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Input {
    warehouse: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parse_warehouse(warehouse_part: &[&str]) -> Vec<Vec<char>> {
    warehouse_part
        .iter()
        .map(|line| line.chars().collect())
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

fn print_warehouse(warehouse: &[Vec<char>]) {
    for row in warehouse {
        for entry in row {
            print!("{}", entry);
        }
        println!();
    }
}

fn get_next_move(curr_pos: (usize, usize), move_dir: &Move) -> (usize, usize) {
    let (curr_x, curr_y) = curr_pos;
    match move_dir {
        Move::Up => (curr_x, curr_y - 1),
        Move::Down => (curr_x, curr_y + 1),
        Move::Left => (curr_x - 1, curr_y),
        Move::Right => (curr_x + 1, curr_y),
    }
}

fn perform_move(warehouse: &mut [Vec<char>], robot_pos: &mut (usize, usize), move_dir: &Move) {
    //println!("Move: {:?}", move_dir);
    let (robot_x, robot_y) = robot_pos;
    let mut maybe_next_move = Some(get_next_move((*robot_x, *robot_y), move_dir));
    while let Some(next_move) = maybe_next_move {
        let (next_x, next_y) = next_move;
        match warehouse[next_y][next_x] {
            '.' => {
                warehouse[*robot_y][*robot_x] = '.';
                warehouse[next_y][next_x] = '@';
                *robot_x = next_x;
                *robot_y = next_y;
                maybe_next_move = None;
            }
            'O' => {
                let (peek_x, peek_y) = get_next_move((next_x, next_y), move_dir);
                match warehouse[peek_y][peek_x] {
                    '.' => {
                        warehouse[next_y][next_x] = '.';
                        warehouse[peek_y][peek_x] = 'O';
                        maybe_next_move = Some(get_next_move((*robot_x, *robot_y), move_dir))
                    }
                    'O' => maybe_next_move = Some((peek_x, peek_y)),
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

fn get_robot_pos(warehouse: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in warehouse.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == '@' {
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

    for move_dir in &input.moves {
        perform_move(&mut warehouse, &mut robot_pos, move_dir);
    }

    warehouse.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .fold(0, |acc, (j, entry)| match entry {
                'O' => acc + (100 * i as u32 + j as u32),
                _ => acc,
            })
    })
}

fn widen_warehouse(warehouse: &[Vec<char>]) -> Vec<Vec<char>> {
    warehouse.iter().fold(vec![], |mut acc, row| {
        let mut new_row: Vec<char> = vec![];
        for entry in row {
            match entry {
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                _ => panic!("Unknown entry"),
            }
        }
        acc.push(new_row);
        acc
    })
}

fn perform_move_wider(
    warehouse: &mut [Vec<char>],
    robot_pos: &mut (usize, usize),
    move_dir: &Move,
) {
    println!("Move: {:?}", move_dir);
    let (robot_x, robot_y) = robot_pos;
    let mut maybe_next_move = Some(get_next_move((*robot_x, *robot_y), move_dir));
    while let Some(next_move) = maybe_next_move {
        let (next_x, next_y) = next_move;
        match warehouse[next_y][next_x] {
            '.' => {
                warehouse[*robot_y][*robot_x] = '.';
                warehouse[next_y][next_x] = '@';
                *robot_x = next_x;
                *robot_y = next_y;
                maybe_next_move = None;
            }
            '[' | ']' => {
                maybe_next_move =
                    perform_move_box_wider(warehouse, (*robot_x, *robot_y), next_move, move_dir);
            }
            _ => {
                maybe_next_move = None;
            }
        }
    }

    print_warehouse(warehouse);
    println!();
}

fn perform_move_box_wider(
    warehouse: &mut [Vec<char>],
    robot_pos: (usize, usize),
    first_box_pos: (usize, usize),
    move_dir: &Move,
) -> Option<(usize, usize)> {
    let mut move_queue: VecDeque<(usize, usize)> = VecDeque::new();
    move_queue.push_front(first_box_pos);

    let mut visited_moves: HashSet<(usize, usize)> = HashSet::new();

    while let Some(curr_move_pos) = move_queue.pop_front() {
        if let Some(adj_moves) = get_adj_moves(curr_move_pos, move_dir) {
            for adj_move_pos in &adj_moves {
                let (curr_move_x, curr_move_y) = curr_move_pos;
                let curr_move = warehouse[curr_move_y][curr_move_x];
                if is_box_move_possible(warehouse, &visited_moves, curr_move_pos) {
                    let (adj_move_x, adj_move_y) = adj_move_pos;
                    let adj_move = warehouse[*adj_move_y][*adj_move_x];
                    println!(
                        "Visiting box {:?} at {:?}, move_dir: {:?}",
                        adj_move, adj_move_pos, move_dir
                    );

                    warehouse[*adj_move_y][*adj_move_x] = curr_move;

                    move_queue.push_front(*adj_move_pos);
                    visited_moves.insert(*adj_move_pos);
                }
            }
        } else {
            continue;
        }
    }
    Some(robot_pos)
}

fn is_box_move_possible(
    warehouse: &[Vec<char>],
    visited_moves: &HashSet<(usize, usize)>,
    curr_move_pos: (usize, usize),
) -> bool {
    let (curr_move_x, curr_move_y) = curr_move_pos;

    // If cell is out of bounds
    if curr_move_x >= warehouse[0].len() || curr_move_y >= warehouse.len() {
        return false;
    }

    // If cell is visited
    if visited_moves.contains(&curr_move_pos) {
        return false;
    }

    // Can't be a wall or a dot
    match warehouse[curr_move_y][curr_move_x] {
        '#' | '.' => return false,
        _ => (),
    }

    // Otherwise can be visited
    return true;
}

fn get_adj_moves(curr_pos: (usize, usize), move_dir: &Move) -> Option<Vec<(usize, usize)>> {
    let (curr_x, curr_y) = curr_pos;
    let mut adj_moves = vec![];

    let potential_moves = match move_dir {
        Move::Up => vec![
            (curr_x - 1, curr_y - 1),
            (curr_x, curr_y - 1),
            (curr_x + 1, curr_y - 1),
        ],
        Move::Down => vec![
            (curr_x - 1, curr_y + 1),
            (curr_x, curr_y + 1),
            (curr_x + 1, curr_y + 1),
        ],
        Move::Left => vec![(curr_x - 1, curr_y)],
        Move::Right => vec![(curr_x + 1, curr_y)],
    };

    for potential_move in potential_moves {
        adj_moves.push(potential_move);
    }

    if adj_moves.is_empty() {
        None
    } else {
        Some(adj_moves)
    }
}

fn get_sum_gps_wider(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut warehouse_wider = widen_warehouse(&input.warehouse);

    let mut robot_pos: (usize, usize) = get_robot_pos(&warehouse_wider);

    println!("Initial robot pos: {:?}", robot_pos);
    println!("Initial state:");
    print_warehouse(&warehouse_wider);
    println!();

    for move_dir in &input.moves {
        perform_move_wider(&mut warehouse_wider, &mut robot_pos, move_dir);
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
    fn test_get_sum_gps_wider_test10() {
        assert_eq!(0, get_sum_gps_wider("input/day15_test10.txt"));
    }

    #[test]
    fn test_get_sum_gps_wider() {
        assert_eq!(0, get_sum_gps_wider("input/day15.txt"));
    }
}
