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

fn print_map(map: &[Vec<MapEntry>]) {
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

struct ListNodePos {
    pri_pos: (usize, usize),
    pri_dir: Direction,
    sec_pos: (usize, usize),
    sec_dir: Direction,
}

fn process_list_entry(
    map: &Vec<Vec<MapEntry>>,
    list_node: &mut ListNode,
    visited_nodes: &mut Vec<(usize, usize)>,
    loop_count: &mut usize,
    list_node_pos: &ListNodePos,
) {
    let (pri_dir_row, pri_dir_col) = list_node_pos.pri_pos;
    let (sec_dir_row, sec_dir_col) = list_node_pos.sec_pos;
    let next_pri_map_entry = map[pri_dir_row][pri_dir_col];
    if next_pri_map_entry == MapEntry::Obstruction {
        /*println!(
            "Found obstruction at pos {:?}, changing direction from {:?} to {:?} at new pos {:?}",
            (pri_dir_row, pri_dir_col), list_node_pos.pri_dir, list_node_pos.sec_dir, (sec_dir_row, sec_dir_col)
        );*/
        let next_sec_map_entry = map[sec_dir_row][sec_dir_col];
        if next_sec_map_entry != MapEntry::Obstruction {
            list_node.next = Some(Box::new(ListNode::new(ListNodeEntry {
                map_entry: next_sec_map_entry,
                pos: (sec_dir_row, sec_dir_col),
                direction: list_node_pos.sec_dir,
            })));
            visited_nodes.push((sec_dir_row, sec_dir_col));
            build_list(
                map,
                list_node.next.as_mut().unwrap(),
                visited_nodes,
                loop_count,
            );
        }
    } else {
        //println!("Maintaining direction {:?}, pos {:?}", list_node_pos.pri_dir, (pri_dir_row, pri_dir_col));
        list_node.next = Some(Box::new(ListNode::new(ListNodeEntry {
            map_entry: next_pri_map_entry,
            pos: (pri_dir_row, pri_dir_col),
            direction: list_node_pos.pri_dir,
        })));
        visited_nodes.push((pri_dir_row, pri_dir_col));
        build_list(
            map,
            list_node.next.as_mut().unwrap(),
            visited_nodes,
            loop_count,
        );
    }
}

fn build_list(
    map: &Vec<Vec<MapEntry>>,
    list_node: &mut ListNode,
    visited_nodes: &mut Vec<(usize, usize)>,
    loop_count: &mut usize,
) {
    let (current_row, current_col) = list_node.val.pos;
    if current_row as i32 - 1 < 0 {
        return;
    }
    if current_col as i32 - 1 < 0 {
        return;
    }
    if current_row as i32 + 1 >= map.len() as i32 {
        return;
    }
    if current_col as i32 + 1 >= map[0].len() as i32 {
        return;
    }
    if detect_loop(visited_nodes) {
        *loop_count += 1;
        /*println!(
            "Detected loop at pos {:?}, loop count {}",
            list_node.val.pos, *loop_count
        );*/
        return;
    }
    let n_dir = (current_row - 1, current_col);
    let s_dir = (current_row + 1, current_col);
    let e_dir = (current_row, current_col + 1);
    let w_dir = (current_row, current_col - 1);
    match list_node.val.direction {
        Direction::N => {
            process_list_entry(
                map,
                list_node,
                visited_nodes,
                loop_count,
                &ListNodePos {
                    pri_pos: n_dir,
                    pri_dir: Direction::N,
                    sec_pos: e_dir,
                    sec_dir: Direction::E,
                },
            );
        }
        Direction::S => {
            process_list_entry(
                map,
                list_node,
                visited_nodes,
                loop_count,
                &ListNodePos {
                    pri_pos: s_dir,
                    pri_dir: Direction::S,
                    sec_pos: w_dir,
                    sec_dir: Direction::W,
                },
            );
        }
        Direction::E => {
            process_list_entry(
                map,
                list_node,
                visited_nodes,
                loop_count,
                &ListNodePos {
                    pri_pos: e_dir,
                    pri_dir: Direction::E,
                    sec_pos: s_dir,
                    sec_dir: Direction::S,
                },
            );
        }
        Direction::W => {
            process_list_entry(
                map,
                list_node,
                visited_nodes,
                loop_count,
                &ListNodePos {
                    pri_pos: w_dir,
                    pri_dir: Direction::W,
                    sec_pos: n_dir,
                    sec_dir: Direction::N,
                },
            );
        }
    }
}

fn get_start_pos(map: &[Vec<MapEntry>]) -> Option<(usize, usize)> {
    for (row, map_col) in map.iter().enumerate() {
        for (col, map_entry) in map_col.iter().enumerate() {
            if *map_entry == MapEntry::GuardN {
                return Some((row, col));
            }
        }
    }
    None
}

fn get_distinct_pos_vec(
    map: &Vec<Vec<MapEntry>>,
    (start_row, start_col): (usize, usize),
) -> (Vec<(usize, usize)>, usize) {
    let mut list_root = ListNode::new(ListNodeEntry {
        map_entry: map[start_row][start_col],
        pos: (start_row, start_col),
        direction: Direction::N,
    });

    let mut visited_nodes: Vec<(usize, usize)> = vec![];
    visited_nodes.push((start_row, start_col));
    let mut loop_count = 0usize;
    build_list(map, &mut list_root, &mut visited_nodes, &mut loop_count);

    let unique_visited_nodes: HashSet<(usize, usize)> = visited_nodes.drain(..).collect();
    (unique_visited_nodes.into_iter().collect(), loop_count)
}

fn get_distinct_pos(input_file: &str) -> usize {
    let input = parse_input(input_file);

    if let Some((start_row, start_col)) = get_start_pos(&input.map) {
        println!("Start node found at pos {:?}", (start_row, start_col));

        get_distinct_pos_vec(&input.map, (start_row, start_col))
            .0
            .len()
    } else {
        panic!("Invalid start node");
    }
}

// Implements Floyd's cycle-finding algorithm using indices
fn detect_loop(visited_nodes: &mut [(usize, usize)]) -> bool {
    // Fast and slow indices initially points to the head
    let mut slow_index = 0;
    let mut fast_index = 0;

    // Loop that runs while fast and slow indexes do not point to mnull null and are not equal
    while fast_index < visited_nodes.len() {
        slow_index += 1;
        fast_index += 2;

        // If fast and slow indices points to the same node,
        // then a cycle is detected
        if slow_index < visited_nodes.len()
            && fast_index < visited_nodes.len()
            && visited_nodes[slow_index] == visited_nodes[fast_index]
        {
            return true;
        }
    }
    false
}

fn get_sum_time_loop_pos(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut sum_time_loop_pos = 0usize;
    if let Some((start_row, start_col)) = get_start_pos(&input.map) {
        println!("Start node found at pos {:?}", (start_row, start_col));

        let res = get_distinct_pos_vec(&input.map, (start_row, start_col));

        println!("Processing {} potential maps ", res.0.len());

        for (visited_row, visited_col) in res.0 {
            let mut pot_map = input.map.clone();
            pot_map[visited_row][visited_col] = MapEntry::Obstruction;

            /*println!(
                "Processing map with new obstruction at {:?}",
                (visited_row, visited_col)
            );*/

            //print_map(&pot_map);

            let new_res = get_distinct_pos_vec(&pot_map, (start_row, start_col));
            sum_time_loop_pos += new_res.1;
        }
    } else {
        panic!("Invalid start node");
    }

    sum_time_loop_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_distinct_pos_vec_test02() {
        let input = parse_input("input/day06_test02.txt");
        assert_eq!(1, get_distinct_pos_vec(&input.map, (6, 4)).1);
    }

    #[test]
    fn test_get_distinct_pos_vec_test03() {
        let input = parse_input("input/day06_test03.txt");
        assert_eq!(1, get_distinct_pos_vec(&input.map, (6, 4)).1);
    }

    #[test]
    fn test_get_distinct_pos_vec_test04() {
        let input = parse_input("input/day06_test04.txt");
        assert_eq!(1, get_distinct_pos_vec(&input.map, (6, 4)).1);
    }

    #[test]
    fn test_get_distinct_pos_vec_test05() {
        let input = parse_input("input/day06_test05.txt");
        assert_eq!(1, get_distinct_pos_vec(&input.map, (6, 4)).1);
    }

    #[test]
    fn test_get_distinct_pos_vec_test06() {
        let input = parse_input("input/day06_test06.txt");
        assert_eq!(1, get_distinct_pos_vec(&input.map, (6, 4)).1);
    }

    #[test]
    fn test_get_distinct_pos_vec_test07() {
        let input = parse_input("input/day06_test07.txt");
        assert_eq!(1, get_distinct_pos_vec(&input.map, (6, 4)).1);
    }

    #[test]
    fn test_get_distinct_pos_test01() {
        assert_eq!(41, get_distinct_pos("input/day06_test01.txt"));
    }

    #[test]
    fn test_get_distinct_pos() {
        // Requires 8MB of stack space
        stacker::grow(8 * 1024 * 1024, || {
            assert_eq!(5564, get_distinct_pos("input/day06.txt"));
        });
    }

    #[test]
    fn test_get_sum_time_loop_pos_test01() {
        assert_eq!(6, get_sum_time_loop_pos("input/day06_test01.txt"));
    }

    #[test]
    fn test_get_sum_time_loop_pos() {
        // Requires 8MB of stack space
        stacker::grow(1024 * 1024 * 1024, || {
            assert_eq!(0, get_sum_time_loop_pos("input/day06.txt"));
        });
    }
}
