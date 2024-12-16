// https://adventofcode.com/2024/day/6

use std::collections::HashSet;

use super::utils::get_lines;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
enum MapEntry {
    #[default]
    Obstruction = b'#',
    GuardN = b'^',
    Clear = b'.',
}

impl TryFrom<u8> for MapEntry {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'#' => Ok(MapEntry::Obstruction),
            b'^' => Ok(MapEntry::GuardN),
            b'.' => Ok(MapEntry::Clear),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

struct Input {
    map: Vec<Vec<MapEntry>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        map: parse_map(iter.next().unwrap().to_owned()),
    }
}

fn parse_map(map_lines: Vec<String>) -> Vec<Vec<MapEntry>> {
    let mut map = vec![];
    for map_line in map_lines.into_iter() {
        let mut map_entries: Vec<MapEntry> = Vec::new();
        for map_entry in map_line.chars() {
            match MapEntry::try_from(map_entry as u8) {
                Ok(pipe) => map_entries.push(pipe),
                Err(_) => panic!("Invalid map entry {}", map_entry),
            }
        }
        map.push(map_entries);
    }
    map
}

fn print_map(map: &Vec<Vec<MapEntry>>) {
    println!("Grid:");
    for map_row in map.iter() {
        for map_entry in map_row {
            print!("{:#}", *map_entry as u8 as char);
        }
        println!();
    }
}

struct ListNodeEntry {
    map_entry: MapEntry,
    pos: (usize, usize),
    direction: Direction,
}

struct ListNode {
    val: ListNodeEntry,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: ListNodeEntry) -> ListNode {
        ListNode { val, next: None }
    }
}

fn process_list_entry(
    map: Vec<Vec<MapEntry>>,
    list_node: &mut ListNode,
    pri_dir: Direction,
    pri_dir_pos: (usize, usize),
    sec_dir: Direction,
    sec_dir_pos: (usize, usize),
) {
    if let Some(next_pri_map_entry) = map.get(pri_dir_pos.0).and_then(|m| m.get(pri_dir_pos.1)) {
        if next_pri_map_entry == &MapEntry::Obstruction {
            if let Some(next_sec_map_entry) =
                map.get(sec_dir_pos.0).and_then(|m| m.get(sec_dir_pos.1))
            {
                println!(
                    "Found obstruction at pos {:?}, changing direction from {:?} to {:?} at new pos {:?}",
                    pri_dir_pos, pri_dir, sec_dir, sec_dir_pos
                );

                if next_sec_map_entry != &MapEntry::Obstruction {
                    list_node.next = Some(Box::new(ListNode::new(ListNodeEntry {
                        map_entry: *next_sec_map_entry,
                        pos: sec_dir_pos,
                        direction: sec_dir,
                    })));
                    build_list(map, list_node.next.as_mut().unwrap());
                }
            }
        } else {
            println!("Maintaining direction {:?}, pos {:?}", pri_dir, pri_dir_pos);
            list_node.next = Some(Box::new(ListNode::new(ListNodeEntry {
                map_entry: *next_pri_map_entry,
                pos: pri_dir_pos,
                direction: pri_dir,
            })));
            build_list(map, list_node.next.as_mut().unwrap());
        }
    }
}

fn build_list(map: Vec<Vec<MapEntry>>, list_node: &mut ListNode) {
    let (current_row, current_col) = list_node.val.pos;
    if current_row as i32 - 1 < 0 {
        return;
    }
    if current_col as i32 - 1 < 0 {
        return;
    }
    if detect_loop(list_node) {
        return;
    }
    let n_dir = (current_row - 1, current_col);
    let s_dir = (current_row + 1, current_col);
    let e_dir = (current_row, current_col + 1);
    let w_dir = (current_row, current_col - 1);
    match list_node.val.direction {
        Direction::N => {
            process_list_entry(map, list_node, Direction::N, n_dir, Direction::E, e_dir);
        }
        Direction::S => {
            process_list_entry(map, list_node, Direction::S, s_dir, Direction::W, w_dir);
        }
        Direction::E => {
            process_list_entry(map, list_node, Direction::E, e_dir, Direction::S, s_dir);
        }
        Direction::W => {
            process_list_entry(map, list_node, Direction::W, w_dir, Direction::N, n_dir);
        }
    }
}

fn get_distinct_pos(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for (col, map_row) in input.map.iter().enumerate() {
        for (row, map_entry) in map_row.iter().enumerate() {
            if *map_entry == MapEntry::GuardN {
                start_pos = Some((col, row));
                break;
            }
        }
    }
    if let Some((row, col)) = start_pos {
        let mut list_root = ListNode::new(ListNodeEntry {
            map_entry: input.map[col][row],
            pos: (row, col),
            direction: Direction::N,
        });

        build_list(input.map, &mut list_root);

        let mut visited_pos: HashSet<(usize, usize)> = HashSet::new();
        visited_pos.insert((row, col));
        let mut iter = &list_root.next;
        while iter.is_some() && iter.as_ref().unwrap().next.is_some() {
            visited_pos.insert(iter.as_ref().unwrap().val.pos);
            iter = &iter.as_ref().unwrap().next;
        }
        // Add one to include to root node
        return visited_pos.len() as u32 + 1;
    } else {
        panic!("Invalid start node");
    }
}

fn detect_loop(list_node: &mut ListNode) -> bool {
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    visited_nodes.insert(list_node.val.pos);

    let mut iter = &list_node.next;
    while iter.is_some() && iter.as_ref().unwrap().next.is_some() {
        if visited_nodes.contains(&iter.as_ref().unwrap().val.pos) {
            return true;
        }
        visited_nodes.insert(iter.as_ref().unwrap().val.pos);
        iter = &iter.as_ref().unwrap().next;
    }

    false
}

fn get_time_loop_pos(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for (col, map_row) in input.map.iter().enumerate() {
        for (row, map_entry) in map_row.iter().enumerate() {
            if *map_entry == MapEntry::GuardN {
                start_pos = Some((col, row));
                break;
            }
        }
    }
    if let Some((row, col)) = start_pos {
        let sum_pot_time_loop_pos = input.map.iter().fold(0, |acc, map_row| {
            acc + map_row.iter().filter(|e| **e == MapEntry::Clear).count()
        });

        let mut visited_map_pos: HashSet<(usize, usize)> = HashSet::new();

        for _ in 0..sum_pot_time_loop_pos {
            let mut new_map = input.map.clone();

            'outer: for (row, map_row) in new_map.iter_mut().enumerate() {
                for (col, map_entry) in map_row.iter_mut().enumerate() {
                    if !visited_map_pos.contains(&(row, col)) {
                        if *map_entry == MapEntry::Clear {
                            *map_entry = MapEntry::Obstruction;
                            visited_map_pos.insert((row, col));
                            break 'outer;
                        }
                    }
                }
            }

            let mut list_root = ListNode::new(ListNodeEntry {
                map_entry: new_map[col][row],
                pos: (row, col),
                direction: Direction::N,
            });

            build_list(new_map, &mut list_root);
        }
    } else {
        panic!("Invalid start node");
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distinct_pos_test01() {
        assert_eq!(41, get_distinct_pos("input/day06_test01.txt"));
    }

    #[test]
    fn test_get_distinct_pos() {
        // 8MB of stack space
        stacker::grow(8 * 1024 * 1024, || {
            assert_eq!(5564, get_distinct_pos("input/day06.txt"));
        });
    }

    #[test]
    fn test_get_time_loop_pos_test01() {
        // 8MB of stack space
        stacker::grow(8 * 1024 * 1024, || {
            assert_eq!(6, get_time_loop_pos("input/day06_test01.txt"));
        });
    }

    #[test]
    fn test_get_time_loop_pos() {
        // 8MB of stack space
        stacker::grow(1024 * 1024 * 1024, || {
            assert_eq!(0, get_time_loop_pos("input/day06.txt"));
        });
    }
}
