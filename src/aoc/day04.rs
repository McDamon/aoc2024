// https://adventofcode.com/2024/day/4

use std::cmp;

use super::utils::get_lines;

struct Input {
    search_grid: Vec<Vec<char>>,
    row_size: usize,
    col_size: usize,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut search_grid: Vec<Vec<char>> = Vec::new();

    let mut row_size: usize = 0;

    for line in lines {
        let row: Vec<char> = line.chars().collect();
        row_size = row.len();
        search_grid.push(row);
    }

    let col_size = search_grid.len();

    Input {
        search_grid,
        row_size,
        col_size,
    }
}

fn print_grid(search_grid: &Vec<Vec<char>>) {
    for row in search_grid {
        let row_str: String = row.into_iter().collect();
        println!("{}", row_str);
    }
}

fn sum_xmas_matches(string_slice: &str) -> u32 {
   let mut sum_xmas = 0;
   sum_xmas += string_slice.matches("XMAS").count() as u32;

   let reverse_string_slice: String = string_slice.chars().rev().collect();
   sum_xmas += reverse_string_slice.matches("XMAS").count() as u32;

   return sum_xmas;
}


fn get_sum_xmas(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut sum_xmas = 0;

    for i in 0..input.row_size {
        let row_str: String = input.search_grid[i].iter().collect();
        sum_xmas += sum_xmas_matches(&row_str);
    }

    for i in 0..input.col_size {
        let col_str: String = input
            .search_grid
            .iter()
            .map(|s| s.iter().nth(i).unwrap())
            .collect();
        sum_xmas += sum_xmas_matches(&col_str);
    }

    let num_diag = input.row_size + input.col_size - 1;
    for i in 0..num_diag {
        let start_row = cmp::min(i, input.row_size - 1);
        let start_col = i - start_row;
        let len = cmp::min(start_row, input.col_size - 1 - start_col) + 1;
        let diag_vec: Vec<char> = (0..len).fold(vec![], |mut data, j| {
            let c = input.search_grid[start_row - j][start_col + j];
            data.push(c);
            data
        });
        let diag_str: String = diag_vec.iter().collect();
        println!("{:?}", diag_str);
        sum_xmas += sum_xmas_matches(&diag_str);
    }

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
