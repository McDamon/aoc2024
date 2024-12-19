// https://adventofcode.com/2024/day/8

use super::utils::get_lines;

struct Input {
    map: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let map_line: Vec<char> = line.chars().collect();
        map.push(map_line);
    }

    Input { map }
}

fn get_unique_antinodes(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut unique_antinodes = 0;

    unique_antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_antinodes_test01() {
        assert_eq!(14, get_unique_antinodes("input/day08_test01.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test02() {
        assert_eq!(2, get_unique_antinodes("input/day08_test02.txt"));
    }
    
    #[test]
    fn test_get_unique_antinodes_test03() {
        assert_eq!(4, get_unique_antinodes("input/day08_test03.txt"));
    }

    #[test]
    fn test_get_unique_antinodes_test04() {
        assert_eq!(3, get_unique_antinodes("input/day08_test04.txt"));
    }

    #[test]
    fn test_get_unique_antinodes() {
        assert_eq!(0, get_unique_antinodes("input/day08.txt"));
    }
}
