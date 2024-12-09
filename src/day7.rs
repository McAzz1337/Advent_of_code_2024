use crate::{PartFn, puzzle_result::PuzzleResult, util::file_io::get_input};

pub fn day7() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    let input = get_input(7);
    PuzzleResult::omitted(7, input, Some(part1), Some(part2))
}

fn prepare(input: &[String]) -> Vec<(usize, Vec<usize>)> {
    input
        .iter()
        .map(|s| {
            let split: Vec<String> = s.split(":").map(|s| s.to_string()).collect();
            let result = split[0].parse::<usize>().unwrap();
            let operands: Vec<usize> = split[1]
                .trim()
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (result, operands)
        })
        .collect()
}

fn add(a: usize, b: usize) -> usize {
    a + b
}
fn mul(a: usize, b: usize) -> usize {
    a * b
}

fn try_solve(
    (result, operands): &(usize, Vec<usize>),
    operators: Vec<fn(usize, usize) -> usize>,
) -> bool {
    if operators.len() == operands.len() - 1 {
        let mut a = operands[0];
        for i in 1..operands.len() {
            let b = operands[i];
            a = (operators[i - 1])(a, b);
        }
        a == *result
    } else {
        try_solve(
            &(*result, operands.clone()),
            [operators.clone(), vec![add]].concat(),
        ) || try_solve(
            &(*result, operands.clone()),
            [operators, vec![mul]].concat(),
        )
    }
}

fn part1(input: &Vec<String>) -> usize {
    let equations = prepare(input);
    equations
        .iter()
        .filter(|e| try_solve(e, vec![]))
        .map(|(r, _)| r)
        .sum()
}

fn concatination(a: usize, b: usize) -> usize {
    let mut a = a.to_string();
    let b = b.to_string();
    a.push_str(b.as_str());
    a.parse::<usize>().unwrap()
}

fn try_solve2(
    (result, operands): &(usize, Vec<usize>),
    operators: Vec<fn(usize, usize) -> usize>,
) -> bool {
    if operators.len() == operands.len() - 1 {
        let mut a = operands[0];
        for i in 1..operands.len() {
            let b = operands[i];
            a = (operators[i - 1])(a, b);
        }
        a == *result
    } else {
        try_solve2(
            &(*result, operands.clone()),
            [operators.clone(), vec![add]].concat(),
        ) || try_solve2(
            &(*result, operands.clone()),
            [operators.clone(), vec![mul]].concat(),
        ) || try_solve2(
            &(*result, operands.clone()),
            [operators, vec![concatination]].concat(),
        )
    }
}

fn part2(input: &Vec<String>) -> usize {
    let equations = prepare(input);
    equations
        .iter()
        .filter(|e| try_solve2(e, vec![]))
        .map(|(r, _)| r)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::util::file_io::get_test_input;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = get_test_input(7);
        let res = part1(&input);
        assert_eq!(res, 3749);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(7);
        let res = part2(&input);
        assert_eq!(res, 11387);
    }
}
