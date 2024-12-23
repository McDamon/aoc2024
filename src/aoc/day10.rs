// https://adventofcode.com/2024/day/10

use super::utils::get_lines;

struct Input {
    top_map: Vec<Vec<u32>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut top_map: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let report: Vec<u32> = line
            .chars()
            .map(|level| level.to_string().parse::<u32>().unwrap())
            .collect();
        top_map.push(report);
    }

    Input { top_map }
}

fn print_top_map(top_map: &Vec<Vec<u32>>) {
    for row in top_map {
        for level in row {
            print!("{:?}", level);
        }
        println!();
    }
}

fn get_sum_trailheads(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    print_top_map(&input.top_map);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sum_trailheads_test01() {
        assert_eq!(1, get_sum_trailheads("input/day10_test01.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test02() {
        assert_eq!(2, get_sum_trailheads("input/day10_test02.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test03() {
        assert_eq!(4, get_sum_trailheads("input/day10_test03.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test04() {
        assert_eq!(3, get_sum_trailheads("input/day10_test04.txt"));
    }

    #[test]
    fn test_get_sum_trailheads_test05() {
        assert_eq!(36, get_sum_trailheads("input/day10_test05.txt"));
    }

    #[test]
    fn test_get_sum_trailheads() {
        assert_eq!(0, get_sum_trailheads("input/day10.txt"));
    }
}
