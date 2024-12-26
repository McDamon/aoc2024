// https://adventofcode.com/2024/day/12

use super::utils::get_lines;

struct Input {
    plants: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut plants: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let report: Vec<char> = line
            .chars()
            .map(|level| level)
            .collect();
        plants.push(report);
    }

    Input { plants }
}

fn print_plants(plants: &Vec<Vec<char>>) {
    for row in plants {
        for level in row {
            print!("{}", level);
        }
        println!();
    }
}

fn get_price_fencing_all_regions(input_file: &str) -> usize {
    let input = parse_input(input_file);

    print_plants(&input.plants);

    let plant_pos_list: Vec<(char, (usize, usize))> = input
        .plants
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_index, plant)| {
                    (*plant, (row_index, col_index))
                })
        })
        .collect::<Vec<(char, (usize, usize))>>();

    println!("{:?}", plant_pos_list);
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_price_fencing_all_regions_test01() {
        assert_eq!(0, get_price_fencing_all_regions("input/day12_test01.txt"));
    }
}
