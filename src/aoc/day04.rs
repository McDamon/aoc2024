// https://adventofcode.com/2024/day/4

use super::utils::get_lines;

struct Input {
    search_grid: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut search_grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        search_grid.push(row);
    }

    Input { search_grid }
}

fn print_grid(search_grid: &Vec<Vec<char>>) {
    for row in search_grid {
        print!("{:?}", row);
    }
}

fn get_sum_xmas(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    print_grid(&input.search_grid);

    let sum_xmas = 0;

    sum_xmas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_xmas_test01() {
        assert_eq!(18, get_sum_xmas("input/day04_test01.txt"));
    }

    #[test]
    fn test_get_sum_xmas() {
        assert_eq!(0, get_sum_xmas("input/day04.txt"));
    }
}
