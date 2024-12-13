use std::{collections::HashSet, usize};

use crate::{
    PartFn,
    puzzle_result::PuzzleResult,
    util::util::{Grid, to_matrix},
};

pub fn day10() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    PuzzleResult::new(10, Some(part1), Some(part2))
}

type Pos = (usize, usize);
type Dir = (isize, isize);

const UP: Dir = (0, -1);
const DOWN: Dir = (0, 1);
const LEFT: Dir = (-1, 0);
const RIGHT: Dir = (1, 0);

fn get_dirs() -> Vec<Dir> {
    vec![UP, RIGHT, DOWN, LEFT]
}

fn find_trail_heads(input: &[String]) -> Vec<Pos> {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| c == &'0')
                .map(|(x, _)| (x, y))
                .collect::<Vec<Pos>>()
        })
        .collect()
}

fn add_dir(pos: Pos, dir: Dir, grid: &Grid) -> Option<Pos> {
    let y = pos.1 as isize + dir.1;
    if y < 0 || y as usize >= grid.len() {
        None
    } else {
        let y = y as usize;
        let x = pos.0 as isize + dir.0;
        if x < 0 || x as usize >= grid[y].len() {
            None
        } else {
            let x = x as usize;
            Some((x, y))
        }
    }
}

fn next_higher(current: &char, next: char) -> bool {
    let current = current.to_digit(10).unwrap();
    let next = next.to_digit(10).unwrap();
    next == current + 1
}

fn is_final_step(current: &char) -> bool {
    current == &'9'
}

fn get_next_step(pos: Pos, grid: &Grid) -> Vec<Dir> {
    let (x, y) = pos;
    let current = grid[y][x];
    let dirs = get_dirs();
    dirs.iter()
        .filter_map(|dir| {
            if let Some(pos) = add_dir(pos, *dir, grid) {
                Some((dir, pos))
            } else {
                None
            }
        })
        .filter(|(_, (x, y))| next_higher(&current, grid[*y][*x]))
        .map(|(dir, _)| dir.clone())
        .collect()
}

fn step(pos: Pos, mut visited: Vec<Pos>, grid: &Grid) -> Vec<Pos> {
    let (x, y) = pos;
    if is_final_step(&grid[y][x]) {
        vec![pos]
    } else {
        let next = get_next_step(pos, grid);
        visited.push(pos);
        let res = next
            .iter()
            .filter_map(|n| {
                if let Some(npos) = add_dir(pos, *n, grid) {
                    Some(step(npos, visited.clone(), grid))
                } else {
                    None
                }
            })
            .flatten()
            .collect::<HashSet<Pos>>();
        res.iter().copied().collect()
    }
}

fn part1(input: &Vec<String>) -> usize {
    let grid = to_matrix(input);
    let heads = find_trail_heads(input);
    heads
        .iter()
        .map(|pos| step(*pos, vec![], &grid))
        .map(|v| v.len())
        .sum::<usize>()
}

fn step2(pos: Pos, mut visited: Vec<Pos>, grid: &Grid) -> usize {
    let (x, y) = pos;
    if is_final_step(&grid[y][x]) {
        1
    } else {
        let next = get_next_step(pos, grid);
        visited.push(pos);
        next.iter()
            .map(|n| {
                if let Some(npos) = add_dir(pos, *n, grid) {
                    step2(npos, visited.clone(), grid)
                } else {
                    0
                }
            })
            .sum()
    }
}

fn part2(input: &Vec<String>) -> usize {
    let grid = to_matrix(input);
    let heads = find_trail_heads(input);
    heads
        .iter()
        .map(|pos| step2(*pos, vec![], &grid))
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::util::{file_io::get_test_input, util::to_matrix};

    use super::{get_next_step, part1, part2, step};

    #[test]
    fn test_part1() {
        let input = get_test_input(10);
        let res = part1(&input);
        assert_eq!(res, 36);
    }

    #[test]
    fn test_next_step() {
        let grid = to_matrix(
            &vec!["010", "101", "010"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        let next = get_next_step((1, 1), &grid);
        assert_eq!(next, vec![(0, -1), (1, 0), (0, 1), (-1, 0)]);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(10);
        let res = part2(&input);
        assert_eq!(res, 81);
    }

    #[test]
    fn test_path() {
        let grid = to_matrix(
            &vec![
                "010000000",
                "023000000",
                "004500000",
                "000678900",
                "000980000",
                "000090000",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        let res = step((0, 0), vec![], &grid);
        assert_eq!(res.len(), 3);
        let res = step((2, 0), vec![], &grid);
        assert_eq!(res.len(), 3);
    }
}
