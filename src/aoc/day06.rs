// https://adventofcode.com/2024/day/6

use std::collections::HashSet;

use grid::Grid;
use indextree::{Arena, NodeId};

use super::utils::get_lines;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
enum GridEntry {
    #[default]
    Obstruction = b'#',
    GuardN = b'^',
    Clear = b'.',
}

impl TryFrom<u8> for GridEntry {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'#' => Ok(GridEntry::Obstruction),
            b'^' => Ok(GridEntry::GuardN),
            b'.' => Ok(GridEntry::Clear),
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
    grid: Grid<GridEntry>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    Input {
        grid: parse_grid(iter.next().unwrap().to_owned()),
    }
}

fn parse_grid(grid_lines: Vec<String>) -> Grid<GridEntry> {
    let mut grid = Grid::new(0, 0);
    for grid_line in grid_lines.into_iter() {
        let mut grid_entries: Vec<GridEntry> = Vec::new();
        for grid_entry in grid_line.chars() {
            match GridEntry::try_from(grid_entry as u8) {
                Ok(pipe) => grid_entries.push(pipe),
                Err(_) => panic!("Invalid grid entry {}", grid_entry),
            }
        }
        grid.push_row(grid_entries)
    }
    grid
}

fn print_grid(grid: &Grid<GridEntry>) {
    println!("Grid:");
    for grid_row in grid.iter_rows() {
        for grid_entry in grid_row {
            print!("{:#}", *grid_entry as u8 as char);
        }
        println!();
    }
}

#[derive(Debug, Copy, Clone)]
struct TreeNodeEntry {
    grid_entry: GridEntry,
    pos: (usize, usize),
    direction: Direction,
}

fn build_tree(
    grid: &Grid<GridEntry>,
    arena: &mut Arena<TreeNodeEntry>,
    current_node_id: NodeId
) {
    let maybe_current_node = arena.get_mut(current_node_id);
    if let Some(current_node) = maybe_current_node {
        let current_node_entry = current_node.get();
        let (current_row, current_col) = current_node_entry.pos;
        if current_row as i32 - 1 < 0 {
            return;
        }
        if current_col as i32 - 1 < 0 {
            return;
        }
        let n_dir = (current_row - 1, current_col);
        let s_dir = (current_row + 1, current_col);
        let e_dir = (current_row, current_col + 1);
        let w_dir = (current_row, current_col - 1);
        match current_node_entry.direction {
            Direction::N => {
                if let Some(next_n_grid_entry) = grid.get(n_dir.0, n_dir.1) {
                    if next_n_grid_entry == &GridEntry::Obstruction {
                        if let Some(next_e_grid_entry) = grid.get(e_dir.0, e_dir.1) {
                            if next_e_grid_entry != &GridEntry::Obstruction {
                                let next_node_id = arena.new_node(TreeNodeEntry {
                                    grid_entry: *next_e_grid_entry,
                                    pos: e_dir,
                                    direction: Direction::E,
                                });
                                current_node_id.append(next_node_id, arena);
                                build_tree(grid, arena, next_node_id);
                            }
                        }
                    } else {
                        let next_node_id = arena.new_node(TreeNodeEntry {
                            grid_entry: *next_n_grid_entry,
                            pos: n_dir,
                            direction: Direction::N,
                        });
                        current_node_id.append(next_node_id, arena);
                        build_tree(grid, arena, next_node_id);
                    }
                }
            }
            Direction::S => {
                if let Some(next_s_grid_entry) = grid.get(s_dir.0, s_dir.1) {
                    if next_s_grid_entry == &GridEntry::Obstruction {
                        if let Some(next_w_grid_entry) = grid.get(w_dir.0, w_dir.1) {
                            if next_w_grid_entry != &GridEntry::Obstruction {
                                let next_node_id = arena.new_node(TreeNodeEntry {
                                    grid_entry: *next_w_grid_entry,
                                    pos: w_dir,
                                    direction: Direction::W,
                                });
                                current_node_id.append(next_node_id, arena);
                                build_tree(grid, arena, next_node_id);
                            }
                        }
                    } else {
                        let next_node_id = arena.new_node(TreeNodeEntry {
                            grid_entry: *next_s_grid_entry,
                            pos: s_dir,
                            direction: Direction::S,
                        });
                        current_node_id.append(next_node_id, arena);
                        build_tree(grid, arena, next_node_id);
                    }
                }
            }
            Direction::E => {
                if let Some(next_e_grid_entry) = grid.get(current_row, current_col + 1) {
                    if next_e_grid_entry == &GridEntry::Obstruction {
                        if let Some(next_s_grid_entry) = grid.get(s_dir.0, s_dir.1) {
                            if next_s_grid_entry != &GridEntry::Obstruction {
                                let next_node_id = arena.new_node(TreeNodeEntry {
                                    grid_entry: *next_s_grid_entry,
                                    pos: s_dir,
                                    direction: Direction::S,
                                });
                                current_node_id.append(next_node_id, arena);
                                build_tree(grid, arena, next_node_id);
                            }
                        }
                    } else {
                        let next_node_id = arena.new_node(TreeNodeEntry {
                            grid_entry: *next_e_grid_entry,
                            pos: e_dir,
                            direction: Direction::E,
                        });
                        current_node_id.append(next_node_id, arena);
                        build_tree(grid, arena, next_node_id);
                    }
                }
            }
            Direction::W => {
                if let Some(next_w_grid_entry) = grid.get(current_row, current_col - 1) {
                    if next_w_grid_entry == &GridEntry::Obstruction {
                        if let Some(next_n_grid_entry) = grid.get(n_dir.0, n_dir.1) {
                            if next_n_grid_entry != &GridEntry::Obstruction {
                                let next_node_id = arena.new_node(TreeNodeEntry {
                                    grid_entry: *next_n_grid_entry,
                                    pos: n_dir,
                                    direction: Direction::N,
                                });
                                current_node_id.append(next_node_id, arena);
                                build_tree(grid, arena, next_node_id);
                            }
                        }
                    } else {
                        let next_node_id = arena.new_node(TreeNodeEntry {
                            grid_entry: *next_w_grid_entry,
                            pos: w_dir,
                            direction: Direction::W,
                        });
                        current_node_id.append(next_node_id, arena);
                        build_tree(grid, arena, next_node_id);
                    }
                }
            }
        }
    }
}

fn get_distinct_pos(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for ((row, col), pipe) in input.grid.indexed_iter() {
        if *pipe == GridEntry::GuardN {
            start_pos = Some((row, col));
        }
    }

    if let Some((row, col)) = start_pos {
        let mut arena: Arena<TreeNodeEntry> = Arena::new();
        let root_node = arena.new_node(TreeNodeEntry {
            grid_entry: GridEntry::GuardN,
            pos: (row, col),
            direction: Direction::N,
        });

        build_tree(&input.grid, &mut arena, root_node);

        let traverser = root_node.traverse(&arena);
        let mut visited_node_ids: HashSet<(usize, usize)> = HashSet::new();
        for ev in traverser {
            match ev {
                indextree::NodeEdge::Start(node_id) => {
                    if let Some(node) = arena.get(node_id) {
                        let node_entry = node.get();
                        visited_node_ids.insert(node_entry.pos);
                    }
                }
                indextree::NodeEdge::End(_) => {}
            }
        }
        visited_node_ids.len() as u32
    } else {
        panic!("Invalid start node");
    }
}

fn get_time_loop_pos(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut start_pos: Option<(usize, usize)> = None;
    for ((row, col), pipe) in input.grid.indexed_iter() {
        if *pipe == GridEntry::GuardN {
            start_pos = Some((row, col));
        }
    }

    if let Some((row, col)) = start_pos {
        let sum_pot_time_loop_pos = input
            .grid
            .iter()
            .filter(|pipe| **pipe == GridEntry::Clear)
            .count();

        let mut visited_grid_pos: HashSet<(usize, usize)> = HashSet::new();

        for time_loop_pos in 0..sum_pot_time_loop_pos {
            let mut grid = input.grid.clone();

            'outer: for i in 0..grid.rows() {
                for j in 0..grid.cols() {
                    if let Some(grid_entry) = grid.get_mut(i, j) {
                        if !visited_grid_pos.contains(&(i, j)) {
                            if *grid_entry == GridEntry::Clear {
                                *grid_entry = GridEntry::Obstruction;
                                visited_grid_pos.insert((i, j));
                                break 'outer;
                            }
                        }
                    }
                }
            }

            let mut arena: Arena<TreeNodeEntry> = Arena::new();
            let root_node = arena.new_node(TreeNodeEntry {
                grid_entry: GridEntry::GuardN,
                pos: (row, col),
                direction: Direction::N,
            });

            println!("Time loop pos: {}", time_loop_pos);
            build_tree(&grid, &mut arena, root_node);
        }

        0
    } else {
        panic!("Invalid start node");
    }
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
        assert_eq!(6, get_time_loop_pos("input/day06_test01.txt"));
    }

    #[test]
    fn test_get_time_loop_pos() {
        // 8MB of stack space
        stacker::grow(1024 * 1024 * 1024, || {
            assert_eq!(0, get_time_loop_pos("input/day06.txt"));
        });
    }
}
