use std::{usize, vec};

use crate::{PartFn, puzzle_result::PuzzleResult, util::file_io::get_input};

pub fn day9() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    let input = get_input(9);
    PuzzleResult::new(9, input, Some(part1), Some(part2))
}

#[derive(Debug)]
struct File {
    id: u32,
    start: usize,
    end: usize,
}

fn parse_input(input: &str) -> Vec<File> {
    let mut is_filename = true;
    let mut filename: u32 = 0;
    let mut ptr: usize = 0;
    let mut files: Vec<File> = Vec::new();

    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;
        if is_filename {
            files.push(File {
                id: filename,
                start: ptr,
                end: ptr + length - 1,
            });

            ptr += length;
            filename += 1;
            is_filename = false;
        } else {
            ptr += length;
            is_filename = true;
        }
    }
    files
}

fn parse_disk(input: &str) -> Vec<Option<u32>> {
    let mut is_filename = true;
    let mut filename: u32 = 0;
    let mut disk: Vec<Option<u32>> = Vec::new();

    for c in input.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;
        let value = if is_filename { Some(filename) } else { None };
        let mut blocks = vec![value; length];
        disk.append(&mut blocks);
        if is_filename {
            filename += 1;
        }
        is_filename = !is_filename;
    }
    disk
}

fn part1(input: &Vec<String>) -> usize {
    let disk = parse_disk(input[0].as_str());
    let mut compressed_disk = disk.clone();
    while compressed_disk.contains(&None) {
        let last = compressed_disk.pop().unwrap();
        if last.is_none() {
            continue;
        }
        if let Some(first_none) = compressed_disk.iter_mut().find(|x| x.is_none()) {
            *first_none = last;
        }
    }
    compressed_disk
        .iter()
        .enumerate()
        .map(|(i, file_id)| i * file_id.unwrap() as usize)
        .sum()
}

fn part2(input: &Vec<String>) -> usize {
    let disk_map = parse_input(input[0].as_str());
    let disk = parse_disk(input[0].as_str());
    let mut compressed_disk = disk.clone();
    for file in disk_map.iter().rev() {
        let right_ptr = file.end;
        let window_size = file.end - file.start + 1;
        if let Some(start_idx) = compressed_disk
            .windows(window_size)
            .position(|window| window == vec![None; window_size])
        {
            if start_idx < right_ptr {
                for i in start_idx..(start_idx + window_size) {
                    compressed_disk[i] = Some(file.id);
                }
                for i in file.start..=file.end {
                    compressed_disk[i] = None;
                }
            }
        }
    }

    compressed_disk
        .iter()
        .enumerate()
        .filter(|(_, x)| x.is_some())
        .map(|(i, file_id)| i * file_id.unwrap() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::file_io::get_test_input;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = get_test_input(9);
        let res = part1(&input);
        assert_eq!(res, 1928);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(9);
        let res = part2(&input);
        assert_eq!(res, 2858);
    }
}
