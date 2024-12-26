// https://adventofcode.com/2024/day/12

use std::collections::{HashMap, HashSet};

use super::utils::get_lines;

struct Input {
    plants: Vec<Vec<char>>,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    let mut plants: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let report: Vec<char> = line.chars().collect();
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

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn new(area: usize, perimeter: usize) -> Region {
        Region { area, perimeter }
    }
}

fn flood_fill(
    plant: char,
    plant_pos: (usize, usize),
    plants: &[Vec<char>],
    regions: &mut HashMap<char, Vec<Region>>,
    visited_plants: &mut HashSet<(usize, usize)>,
) {
    let mut stack: Vec<(usize, usize)> = Vec::new();

    // Add current node to the stack
    stack.push(plant_pos);

    let mut area = 0;
    let mut perimeter = 0;

    while let Some((row, col)) = stack.pop() {
        // This makes sure we don't double count positions
        if visited_plants.contains(&(row, col)) {
            continue;
        }

        // Is this 'Inside'?
        if plants[row][col] == plant {
            // If so, increment the area, and mark as visited
            area += 1;
            visited_plants.insert((row, col));

            // Check is area N is a perimeter
            if row as i32 - 1 < 0 || plants[row - 1][col] != plant {
                perimeter += 1;
            }
            // Check if area S is a perimeter
            if row as i32 + 1 >= plants.len() as i32 || plants[row + 1][col] != plant {
                perimeter += 1;
            }
            // Check if area E is a perimeter
            if col as i32 + 1 >= plants[0].len() as i32 || plants[row][col + 1] != plant {
                perimeter += 1;
            }
            // Check if area W is a perimeter
            if col as i32 - 1 < 0 || plants[row][col - 1] != plant {
                perimeter += 1;
            }

            if row > 0 {
                // N direction
                stack.push((row - 1, col));
            }
            if row < plants.len() - 1 {
                // S direction
                stack.push((row + 1, col));
            }
            if col < plants[0].len() - 1 {
                // E direction
                stack.push((row, col + 1));
            }
            if col > 0 {
                // W direction
                stack.push((row, col - 1));
            }
        }
    }

    if area > 0 && perimeter > 0 {
        // Once we are done, we can add the new region
        regions
            .entry(plant)
            .or_default()
            .push(Region::new(area, perimeter));
    }
}

fn get_price_fencing_all_regions(input_file: &str) -> usize {
    let input = parse_input(input_file);

    //print_plants(&input.plants);

    let plant_pos_list: Vec<(char, (usize, usize))> = input
        .plants
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_index, plant)| (*plant, (row_index, col_index)))
        })
        .collect::<Vec<(char, (usize, usize))>>();

    let mut regions: HashMap<char, Vec<Region>> = HashMap::new();
    let mut visited_plants: HashSet<(usize, usize)> = HashSet::new();

    for (plant, (row, col)) in plant_pos_list {
        flood_fill(
            plant,
            (row, col),
            &input.plants,
            &mut regions,
            &mut visited_plants,
        );
    }

    //println!("Regions: {:?}", regions);

    regions.into_values().map(|region| {
            region
                .into_iter()
                .map(|r| r.area * r.perimeter)
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_price_fencing_all_regions_test01() {
        assert_eq!(140, get_price_fencing_all_regions("input/day12_test01.txt"));
    }

    #[test]
    fn test_get_price_fencing_all_regions_test02() {
        assert_eq!(772, get_price_fencing_all_regions("input/day12_test02.txt"));
    }

    #[test]
    fn test_get_price_fencing_all_regions_test03() {
        assert_eq!(
            1930,
            get_price_fencing_all_regions("input/day12_test03.txt")
        );
    }

    #[test]
    fn test_get_price_fencing_all_regions() {
        assert_eq!(1363682, get_price_fencing_all_regions("input/day12.txt"));
    }
}
