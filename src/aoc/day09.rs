// https://adventofcode.com/2024/day/9

use super::utils::get_lines;

struct Input {
    disk: Vec<DiskEntry>,
}

#[derive(Debug)]
enum DiskEntryType {
    File,
    FreeSpace,
}

#[derive(Debug)]
struct DiskEntry {
    id: usize,
    entry: DiskEntryType,
    len: usize,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    if let Some(line) = lines.first() {
        let chars: Vec<char> = line.chars().collect();

        let mut id = 0usize;
        let disk: Vec<DiskEntry> = chars
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 != 0 {
                    id += 1;
                }
                DiskEntry {
                    id: id,
                    entry: if i % 2 == 0 {
                        DiskEntryType::File
                    } else {
                        DiskEntryType::FreeSpace
                    },
                    len: c.to_string().parse::<usize>().unwrap_or(0),
                }
            })
            .collect();
        Input { disk }
    } else {
        panic!("Invalid input file: {}", input_file);
    }
}

fn get_blocks(disk: &Vec<DiskEntry>) -> String {
    let mut blocks = String::new();
    for entry in disk {
        match entry.entry {
            DiskEntryType::File => {
                blocks.push_str(&entry.id.to_string().repeat(entry.len));
            }
            DiskEntryType::FreeSpace => {
                blocks.push_str(&".".repeat(entry.len));
            }
        }
    }
    blocks
}

fn is_file_block_gaps(blocks: &str) -> bool {
    let free_space_count = blocks.chars().rev().take_while(|&c| c == '.').count();
    free_space_count != blocks.chars().filter(|&c| c == '.').count()
}

fn calc_checksum(blocks: &str) -> u32 {
    let mut checksum = 0u32;
    for (i, c) in blocks.chars().enumerate() {
        if c.is_ascii_digit() {
            checksum += i as u32 * c.to_digit(10).unwrap();
        }
    }
    checksum
}

fn get_checksum(input_file: &str) -> u32 {
    let input = parse_input(input_file);

    let mut blocks = get_blocks(&input.disk);

    loop {
        println!("{}", blocks);
        if is_file_block_gaps(&blocks) {
            if let Some(first_free_space_block_pos) = blocks.find(".") {
                if let Some(last_file_block_pos) = blocks.rfind(|c: char| c.is_ascii_digit()) {
                    let first_free_space_block =
                        blocks.chars().nth(first_free_space_block_pos).unwrap();
                    let last_file_block = blocks.chars().nth(last_file_block_pos).unwrap();

                    let mut blocks_vec: Vec<char> = blocks.chars().collect();
                    blocks_vec[first_free_space_block_pos] = last_file_block;
                    blocks_vec[last_file_block_pos] = first_free_space_block;
                    blocks = blocks_vec.into_iter().collect();
                }
            }
        } else {
            break;
        }
    }

    calc_checksum(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_checksum_test01() {
        assert_eq!(1928, get_checksum("input/day09_test01.txt"));
    }

    #[test]
    fn test_get_checksum_test02() {
        assert_eq!(60, get_checksum("input/day09_test02.txt"));
    }

    #[test]
    fn test_get_checksum_test03() {
        assert_eq!(513, get_checksum("input/day09_test03.txt"));
    }

    #[test]
    fn test_get_checksum() {
        assert_eq!(0, get_checksum("input/day09.txt"));
    }
}
