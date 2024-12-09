use crate::{PartFn, puzzle_result::PuzzleResult, util::file_io::get_input};

pub fn day2() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    let input = get_input(2);
    PuzzleResult::new(2, input, Some(part1), Some(part2))
}

fn line_to_vec_usize(s: &String) -> Vec<usize> {
    s.split(" ").map(|s| s.parse::<usize>().unwrap()).collect()
}

fn is_positave_delta(a: usize, b: &usize) -> bool {
    a as isize - *b as isize >= 0
}

fn is_safe(report: &Vec<usize>) -> bool {
    let delta = is_positave_delta(report[0], &report[1]);
    let same = |a: usize, b: &usize| {
        let delta1 = is_positave_delta(a, b);
        delta == delta1
    };
    let diff = |a: usize, b: &usize| {
        let diff = (a as isize - *b as isize).abs();
        diff > 0 && diff < 4
    };
    report
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| same(report[i - 1], x) && diff(report[i - 1], x))
        .all(|b| b)
}

fn part1(input: &Vec<String>) -> usize {
    let reports: Vec<Vec<usize>> = input.iter().map(line_to_vec_usize).collect();
    reports.iter().map(is_safe).filter(|b| *b).count()
}

fn try_make_safe(report: &Vec<usize>) -> Vec<usize> {
    let safe: Vec<Vec<usize>> = (0..report.len())
        .map(|i| {
            let mut report = report.clone();
            report.remove(i);
            report
        })
        .map(|r| (is_safe(&r), r))
        .filter(|(b, _)| *b)
        .map(|(_, r)| r)
        .collect();

    if let Some(safe) = safe.first() {
        safe.clone()
    } else {
        report.clone()
    }
}

fn make_safe(report: &Vec<usize>) -> Vec<usize> {
    if is_safe(report) {
        report.clone()
    } else {
        try_make_safe(report)
    }
}

fn part2(input: &Vec<String>) -> usize {
    let reports: Vec<Vec<usize>> = input.iter().map(line_to_vec_usize).collect();
    reports.iter().map(make_safe).filter(is_safe).count()
}

#[cfg(test)]
mod tests {
    use crate::util::file_io::get_test_input;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = get_test_input(2);
        let res = part1(&input);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(2);
        let res = part2(&input);
        assert_eq!(res, 4);
    }
}
