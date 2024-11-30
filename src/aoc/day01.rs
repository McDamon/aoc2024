// https://adventofcode.com/2024/day/1

use super::utils::get_lines;

fn get_placeholder(input_file: &str) -> u32 {
    let lines = get_lines(input_file);

    let placeholder = 10;

    for line in lines {
        if !line.is_empty() {
            println!("{}", line)
        }
    }

    placeholder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_test01() {
        assert_eq!(
            10,
            get_placeholder("input/day01.txt")
        );
    }
}