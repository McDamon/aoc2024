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

fn print_grid(grid: &Vec<Vec<GridEntry>>) {
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
    mut_area: &mut Vec<Vec<GridEntry>>,
    current_pos: &mut (usize, usize),
) -> bool {
    mut_area[current_pos.0][current_pos.1] = GridEntry::Visited;
    print_grid(&mut_area);
    true
}

fn get_distinct_pos(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut distinct_pos: u32 = 0;

    let mut mut_area = input.mapped_area.clone();
    let mut current_pos = input.start_pos;
    loop {
        if !guard_leaves_mapped_area(&mut mut_area, &mut current_pos) {
            for i in 0..mut_area.len() {
                for j in 0..mut_area[i].len() {
                    if mut_area[i][j] == GridEntry::Visited {
                        distinct_pos += 1
                    }
                }
            }
        } else {
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
        assert_eq!(0, get_distinct_pos("input/day06.txt"));
    }
}
