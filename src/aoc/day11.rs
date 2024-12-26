// https://adventofcode.com/2024/day/11

use super::utils::get_lines;

use itertools::Itertools;

struct Input {
    stones: Vec<usize>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut stones: Vec<usize> = Vec::new();

    for line in lines {
        let line_parts: Vec<&str> = line.split_whitespace().collect();
        for line_part in line_parts {
            stones.push(line_part.parse::<usize>().unwrap());
        }
    }

    Input { stones }
}

fn print_stones(stones: &Vec<usize>) {
    println!("{}", stones.iter().format(" "));
}

fn get_new_stones(stone: usize) -> Vec<usize> {
    let mut new_stones: Vec<usize> = Vec::new();
    match stone {
        0 => {
            // Replace with stone engraved with the number 1
            new_stones.push(1);
        }
        _ if stone.to_string().len() % 2 == 0 => {
            // Replace with two stones (split each number string in half)
            let stone_str = stone.to_string();
            let (first_half, second_half) = stone_str.split_at(stone_str.len() / 2);
            new_stones.push(first_half.parse::<usize>().unwrap());
            new_stones.push(second_half.parse::<usize>().unwrap());
        }
        _ => {
            // Replace with stone multiplied by 2024
            new_stones.push(stone * 2024);
        }
    }
    return new_stones;
}

fn get_num_stones(input_file: &str, blinks: usize) -> usize {
    let input = parse_input(input_file);

    println!("Initial arrangement:");
    print_stones(&input.stones);
    println!("");

    let mut stones: Vec<usize> = input.stones.clone();

    for _ in 0..blinks {
        let mut new_stones: Vec<usize> = Vec::new();

        for (_, stone) in stones.iter().enumerate() {
            new_stones.extend(get_new_stones(*stone));
        }
        stones = new_stones;

        print_stones(&stones);
    }

    stones.len()
}

fn process_stone(stone: usize, blinks: usize, blink_count: &mut usize, num_stones: &mut usize) {
    if *blink_count >= blinks {
        *num_stones += 1;
        return;
    }
    let new_stones: Vec<usize> = get_new_stones(stone);

    println!("##########");
    println!("Stone: {}", stone);
    println!("New stones: {:?}", new_stones);
    println!("Blink count: {}", blink_count);
    println!("Num stones: {}", num_stones);
    println!("##########");

    for stone in new_stones {
        process_stone(stone, blinks, &mut (*blink_count + 1), num_stones);
    }
}

fn get_num_stones_memoize(input_file: &str, blinks: usize) -> usize {
    let input = parse_input(input_file);

    println!("Initial arrangement:");
    print_stones(&input.stones);
    println!("");

    let stones: Vec<usize> = input.stones.clone();

    let mut num_stones = 0;

    for stone in stones {
        let mut blink_count = 0;
        process_stone(stone, blinks, &mut blink_count, &mut num_stones);
    }

    num_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num_stones_test01() {
        assert_eq!(7, get_num_stones("input/day11_test01.txt", 1));
    }

    #[test]
    fn test_get_num_stones_test02() {
        assert_eq!(3, get_num_stones("input/day11_test02.txt", 1));
    }

    #[test]
    fn test_get_num_stones_test03() {
        assert_eq!(4, get_num_stones("input/day11_test02.txt", 2));
    }

    #[test]
    fn test_get_num_stones_test04() {
        assert_eq!(5, get_num_stones("input/day11_test02.txt", 3));
    }

    #[test]
    fn test_get_num_stones_test05() {
        assert_eq!(9, get_num_stones("input/day11_test02.txt", 4));
    }

    #[test]
    fn test_get_num_stones_test06() {
        assert_eq!(13, get_num_stones("input/day11_test02.txt", 5));
    }

    #[test]
    fn test_get_num_stones_test07() {
        assert_eq!(22, get_num_stones("input/day11_test02.txt", 6));
    }

    #[test]
    fn test_get_num_stones_test08() {
        assert_eq!(55312, get_num_stones("input/day11_test02.txt", 25));
    }

    #[test]
    fn test_get_num_stones() {
        assert_eq!(187738, get_num_stones("input/day11.txt", 25));
    }

    #[test]
    fn test_get_num_stones_memoize_test01() {
        assert_eq!(7, get_num_stones_memoize("input/day11_test01.txt", 1));
    }

    #[test]
    fn test_get_num_stones_memoize_test02() {
        assert_eq!(3, get_num_stones_memoize("input/day11_test02.txt", 1));
    }

    #[test]
    fn test_get_num_stones_memoize_test03() {
        assert_eq!(4, get_num_stones_memoize("input/day11_test02.txt", 2));
    }

    #[test]
    fn test_get_num_stones_memoize_test04() {
        assert_eq!(5, get_num_stones_memoize("input/day11_test02.txt", 3));
    }

    #[test]
    fn test_get_num_stones_memoize_test05() {
        assert_eq!(9, get_num_stones_memoize("input/day11_test02.txt", 4));
    }

    #[test]
    fn test_get_num_stones_memoize_test06() {
        assert_eq!(13, get_num_stones_memoize("input/day11_test02.txt", 5));
    }

    #[test]
    fn test_get_num_stones_memoize_test07() {
        assert_eq!(22, get_num_stones_memoize("input/day11_test02.txt", 6));
    }

    #[test]
    fn test_get_num_stones_memoize_test08() {
        assert_eq!(55312, get_num_stones_memoize("input/day11_test02.txt", 25));
    }

    #[test]
    fn test_get_num_stones_memoize_25_blinks() {
        assert_eq!(187738, get_num_stones_memoize("input/day11.txt", 25));
    }

    #[test]
    fn test_get_num_stones_memoize_75_blinks() {
        assert_eq!(0, get_num_stones_memoize("input/day11.txt", 75));
    }
}
