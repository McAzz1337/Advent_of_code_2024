use std::usize;

use crate::{PartFn, puzzle_result::PuzzleResult, util::file_io::get_input};

use regex::Regex;

pub fn day3() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    let input = get_input(3);
    PuzzleResult::new(3, input, Some(part1), Some(part2))
}

fn extract_numbers(s: &str) -> (usize, String) {
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    if let Some(reg) = regex.captures(s) {
        let m = reg
            .iter()
            .filter_map(|cap| cap.map(|m| m.as_str()))
            .collect::<Vec<&str>>()
            .join(" ");
        let start = m.find("(").unwrap() + 1;
        let end = m.find(")").unwrap();
        let m = &m[start..end];
        let nums: Vec<usize> = m.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        let end = reg.get(0).map(|m| m.end()).unwrap();
        (nums[0] * nums[1], s[end..].to_string())
    } else {
        (0, String::new())
    }
}

fn process_line(s: &String) -> usize {
    let mut s = s.clone();
    let mut acc = 0;
    while !s.is_empty() {
        let (num, new_str) = extract_numbers(&s);
        acc += num;
        s = new_str;
    }
    acc
}

fn part1(input: &Vec<String>) -> usize {
    input.iter().map(process_line).sum()
}

fn take_from_to(s: &String, start: &str, end: &str) -> (String, String) {
    let start_reg = Regex::new(start).unwrap();
    let end_reg = Regex::new(end).unwrap();
    if let Some(reg) = start_reg.captures(s) {
        let i = reg.get(0).map(|m| m.start()).unwrap();
        if let Some(reg) = end_reg.captures(s) {
            let j = reg.get(0).map(|m| m.start()).unwrap();
            (s[i..j].to_string(), s[j + 5..].to_string())
        } else {
            (s[i..].to_string(), String::new())
        }
    } else {
        (String::new(), String::new())
    }
}

fn get_enabled_sections(mut s: String) -> String {
    s = String::from("do") + s.as_str();
    let mut res = String::new();
    while !s.is_empty() {
        let (take, rest) = take_from_to(&s, "do", "don't");
        res.push_str(take.as_str());
        s = rest;
    }
    res
}

fn part2(input: &Vec<String>) -> usize {
    let input = input.iter().fold(String::new(), |a, b| a + b.as_str());
    let input = get_enabled_sections(input);
    process_line(&input)
}

#[cfg(test)]
mod tests {

    use regex::Regex;

    use super::{part1, part2};
    use crate::util::file_io::get_test_input;

    #[test]
    fn test_part1() {
        let input = get_test_input(3);
        let res = part1(&input);
        assert_eq!(res, 161);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(3);
        let res = part2(&input);
        assert_eq!(res, 48);
    }

    #[test]
    fn test_reg() {
        let start = Regex::new("do").unwrap();
        let end = Regex::new("don't").unwrap();
        let input = get_test_input(3);
        if let Some(s) = start.captures(input[0].as_str()) {
            let i = s.get(0).map(|m| m.start()).unwrap();
            assert_eq!(&input[0][i..i + 2], "do");
        }
        if let Some(s) = end.captures(input[0].as_str()) {
            let i = s.get(0).map(|m| m.start()).unwrap();
            assert_eq!(&input[0][i..i + 5], "don't");
        }
    }
}
