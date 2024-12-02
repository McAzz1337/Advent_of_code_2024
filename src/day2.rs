use crate::util::file_io::get_input;

pub fn day2() {
    println!("day 2:");
    let input = get_input(2);
    let res = part1(&input);
    println!("part1 {res}");
    let res = part2(&input);
    println!("part2 {res}");
}

fn line_to_vec_i32(s: &String) -> Vec<i32> {
    s.split(" ").map(|s| s.parse::<i32>().unwrap()).collect()
}

fn is_positave_delta(a: i32, b: &i32) -> bool {
    a - *b >= 0
}

fn is_safe(report: &Vec<i32>) -> bool {
    let delta = is_positave_delta(report[0], &report[1]);
    let same = |a: i32, b: &i32| {
        let delta1 = is_positave_delta(a, b);
        delta == delta1
    };
    let diff = |a: i32, b: &i32| {
        let diff = (a - *b).abs();
        diff > 0 && diff < 4
    };
    report
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| same(report[i - 1], x) && diff(report[i - 1], x))
        .all(|b| b)
}

fn part1(input: &Vec<String>) -> i32 {
    let reports: Vec<Vec<i32>> = input.iter().map(line_to_vec_i32).collect();
    reports.iter().map(is_safe).filter(|b| *b).count() as i32
}

fn try_make_safe(report: &Vec<i32>) -> Vec<i32> {
    let safe: Vec<Vec<i32>> = (0..report.len())
        .into_iter()
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

fn make_safe(report: &Vec<i32>) -> Vec<i32> {
    if is_safe(report) {
        report.clone()
    } else {
        try_make_safe(report)
    }
}

fn part2(input: &Vec<String>) -> i32 {
    let reports: Vec<Vec<i32>> = input.iter().map(line_to_vec_i32).collect();
    reports.iter().map(make_safe).filter(is_safe).count() as i32
}

#[cfg(test)]
mod testpart2 {
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
