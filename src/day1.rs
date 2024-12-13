use crate::{PartFn, puzzle_result::PuzzleResult};

pub fn day1() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    PuzzleResult::new(1, Some(part1), Some(part2))
}

fn split_line(s: &String) -> (String, String) {
    let left = s.chars().take_while(|c| c.is_digit(10)).collect::<String>();
    let right = s
        .chars()
        .skip(left.len())
        .skip_while(|c| !c.is_digit(10))
        .collect::<String>();
    (left, right)
}

fn part1(input: &Vec<String>) -> usize {
    let split = input
        .iter()
        .map(split_line)
        .collect::<Vec<(String, String)>>();

    let mut left = split
        .iter()
        .map(|(s, _)| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut right = split
        .iter()
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (*a as isize - *b as isize).unsigned_abs())
        .sum()
}

fn count_in(x: &usize, v: &[usize]) -> usize {
    v.iter().filter(|y| *y == x).count()
}

fn part2(input: &Vec<String>) -> usize {
    let split: Vec<(String, String)> = input.iter().map(split_line).collect();
    let left = split
        .iter()
        .map(|(s, _)| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let right = split
        .iter()
        .map(|(_, s)| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    left.iter()
        .map(|x| (x, count_in(x, &right)))
        .map(|(x, y)| x * y)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::file_io::get_test_input;

    use super::{part1, part2, split_line};

    #[test]
    fn test_split_line() {
        let input = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];
        let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        let res = split_line(&input[0]);
        assert_eq!(res, (3.to_string(), 4.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = get_test_input(1);
        let res = part1(&input);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(1);
        let res = part2(&input);
        assert_eq!(res, 31);
    }
}
