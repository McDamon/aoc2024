// https://adventofcode.com/2024/day/6

use super::utils::get_lines;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
enum GridEntry {
    #[default]
    Obstruction,
    GuardN,
    GuardS,
    GuardE,
    GuardW,
    Visited,
    Clear,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

struct Input {
    mapped_area: Vec<Vec<GridEntry>>,
    start_pos: (usize, usize),
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut iter = lines.split(|e| e.is_empty());

    let grid = parse_grid(iter.next().unwrap().to_owned());

    Input {
        mapped_area: grid.0,
        start_pos: grid.1,
    }
}

fn parse_grid(tiles_lines: Vec<String>) -> (Vec<Vec<GridEntry>>, (usize, usize)) {
    let mut tiles = vec![];
    let (mut row, mut col) = (0, 0);
    for (i, tiles_line) in tiles_lines.iter().enumerate() {
        let mut tiles_entries: Vec<GridEntry> = Vec::new();
        for (j, tiles_entry) in tiles_line.chars().enumerate() {
            match tiles_entry {
                '^' => {
                    tiles_entries.push(GridEntry::GuardN);
                    row = i;
                    col = j;
                }
                'v' => tiles_entries.push(GridEntry::GuardS),
                '>' => tiles_entries.push(GridEntry::GuardE),
                '<' => tiles_entries.push(GridEntry::GuardW),
                '.' => tiles_entries.push(GridEntry::Clear),
                _ => tiles_entries.push(GridEntry::Obstruction),
            }
        }
        tiles.push(tiles_entries)
    }
    (tiles, (row, col))
}

fn print_grid(grid: &[Vec<GridEntry>]) {
    println!("Grid:");
    for row in grid.iter() {
        for entry in row {
            match entry {
                GridEntry::Obstruction => print!("#"),
                GridEntry::GuardN => print!("^"),
                GridEntry::GuardS => print!("v"),
                GridEntry::GuardE => print!(">"),
                GridEntry::GuardW => print!("<"),
                GridEntry::Visited => print!("X"),
                GridEntry::Clear => print!("."),
            }
        }
        println!();
    }
}

fn guard_leaves_mapped_area(
    mut_area: &mut [Vec<GridEntry>],
    (current_row, current_col): &mut (usize, usize),
    current_dir: &mut Direction,
) -> bool {
    let current_entry = &mut mut_area[*current_row][*current_col];
    match current_dir {
        Direction::N => {
            *current_entry = GridEntry::Visited;
            if *current_row == 0 {
                return true;
            }
            *current_row -= 1;

            let next_entry = &mut mut_area[*current_row][*current_col];

            if next_entry == &GridEntry::Obstruction {
                *current_dir = Direction::E;
                *current_row += 1;
            }
        }
        Direction::S => {
            *current_entry = GridEntry::Visited;
            if *current_row == mut_area.len() - 1 {
                return true;
            }
            *current_row += 1;

            let next_entry = &mut mut_area[*current_row][*current_col];

            if next_entry == &GridEntry::Obstruction {
                *current_dir = Direction::W;
                *current_row -= 1;
            }
        }
        Direction::E => {
            *current_entry = GridEntry::Visited;
            if *current_col == mut_area[*current_row].len() - 1 {
                return true;
            }
            *current_col += 1;

            let next_entry = &mut mut_area[*current_row][*current_col];

            if next_entry == &GridEntry::Obstruction {
                *current_dir = Direction::S;
                *current_col -= 1;
            }
        }
        Direction::W => {
            *current_entry = GridEntry::Visited;
            if *current_col == 0 {
                return true;
            }
            *current_col -= 1;

            let next_entry = &mut mut_area[*current_row][*current_col];

            if next_entry == &GridEntry::Obstruction {
                *current_dir = Direction::N;
                *current_col += 1;
            }
        }
    }
    //print_grid(mut_area);
    false
}

fn get_distinct_pos(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut distinct_pos: u32 = 0;

    let mut mut_area = input.mapped_area.clone();
    let mut current_pos = input.start_pos;
    let mut current_dir: Direction = Direction::N;
    loop {
        if guard_leaves_mapped_area(&mut mut_area, &mut current_pos, &mut current_dir) {
            for (row, mut_area_row) in mut_area.iter().enumerate() {
                for (col, _) in mut_area_row.iter().enumerate() {
                    if mut_area[row][col] == GridEntry::Visited {
                        distinct_pos += 1
                    }
                }
            }
            break;
        }
    }
    distinct_pos
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
        assert_eq!(5564, get_distinct_pos("input/day06.txt"));
    }
}
