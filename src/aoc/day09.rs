// https://adventofcode.com/2024/day/9

use super::utils::get_lines;

struct Input {
    disk: Vec<DiskEntry>,
}

#[derive(Debug, Clone, PartialEq)]
enum DiskEntryType {
    File,
    FreeSpace,
}

#[derive(Debug, Clone)]
struct DiskEntry {
    id: Option<usize>,
    entry: DiskEntryType,
}

fn parse_input(input_file: &str) -> Input {
    let lines = get_lines(input_file);

    if let Some(line) = lines.first() {
        let chars: Vec<char> = line.chars().collect();

        let mut id = 0usize;
        let disk: Vec<DiskEntry> =
            chars
                .into_iter()
                .enumerate()
                .fold(vec![], |mut acc: Vec<DiskEntry>, (i, c)| {
                    if i % 2 != 0 {
                        id += 1;
                    }
                    for _ in 0..c.to_digit(10).unwrap_or(0) {
                        acc.push(DiskEntry {
                            id: if i % 2 == 0 { Some(id) } else { None },
                            entry: if i % 2 == 0 {
                                DiskEntryType::File
                            } else {
                                DiskEntryType::FreeSpace
                            },
                        });
                    }
                    acc
                });
        Input { disk }
    } else {
        panic!("Invalid input file: {}", input_file);
    }
}

fn find_first_free_space_block(blocks: &Vec<DiskEntry>) -> Option<usize> {
    blocks
        .iter()
        .position(|block| matches!(block.entry, DiskEntryType::FreeSpace))
}

fn find_last_file_block(blocks: &Vec<DiskEntry>) -> Option<usize> {
    blocks
        .iter()
        .rposition(|block| matches!(block.entry, DiskEntryType::File))
}

fn has_file_block_gaps(blocks: &Vec<DiskEntry>) -> bool {
    let free_space_count = blocks
        .iter()
        .rev()
        .take_while(|block| block.entry == DiskEntryType::FreeSpace)
        .count();
    free_space_count
        != blocks
            .into_iter()
            .filter(|block| block.entry == DiskEntryType::FreeSpace)
            .count()
}

fn calc_checksum(blocks: &Vec<DiskEntry>) -> usize {
    blocks
        .into_iter()
        .enumerate()
        .filter(|(_, block)| block.entry != DiskEntryType::FreeSpace)
        .filter_map(|(i, block)| Some(i as usize * block.id.unwrap_or(0) as usize))
        .sum()
}

fn get_checksum(input_file: &str) -> usize {
    let input = parse_input(input_file);

    let mut blocks = input.disk.clone();

    loop {
        if has_file_block_gaps(&blocks) {
            if let Some(first_free_space_block_pos) = find_first_free_space_block(&blocks) {
                if let Some(last_file_block_pos) = find_last_file_block(&blocks) {
                    let first_free_space_block = blocks[first_free_space_block_pos].clone();
                    let last_file_block = blocks[last_file_block_pos].clone();

                    blocks[first_free_space_block_pos] = last_file_block.clone();
                    blocks[last_file_block_pos] = first_free_space_block.clone();
                }
            }
        } else {
            break;
        }
    }

    //println!("{:?}", blocks);

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
    fn test_get_checksum_test04() {
        assert_eq!(57, get_checksum("input/day09_test04.txt"));
    }

    #[test]
    fn test_get_checksum() {
        assert_eq!(0, get_checksum("input/day09.txt"));
    }
}
