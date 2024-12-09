use core::panic;
use std::collections::HashSet;

use crate::{
    PartFn,
    puzzle_result::PuzzleResult,
    util::{
        file_io::get_input,
        util::{Grid, to_matrix},
    },
};

type Pos = (isize, isize);
type Dir = (isize, isize);

pub fn day6() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    let input = get_input(6);
    PuzzleResult::omitted(6, input, Some(part1), Some(part2))
}

struct Guard<F: Fn(Pos, Dir, &mut Grid)> {
    x: isize,
    y: isize,
    dir: Dir,
    replace: F,
}

impl<F: Fn(Pos, Dir, &mut Grid)> Guard<F> {
    fn new(grid: &Grid, replace: F) -> Guard<F> {
        let (x, y) = Self::get_guard_pos(grid);
        let sym = grid[y as usize][x as usize];
        Guard {
            x,
            y,
            dir: Self::get_dir(sym),
            replace,
        }
    }

    fn from(x: isize, y: isize, dir: Dir, replace: F) -> Guard<F> {
        Guard { x, y, dir, replace }
    }

    fn step(&mut self, grid: &mut Grid) -> bool {
        if self.hit_obstacle(grid) {
            (self.replace)((self.x, self.y), self.dir, grid);
            self.rotate();
            (self.replace)((self.x, self.y), self.dir, grid);
            true
        } else if self.hit_wall(grid) {
            (self.replace)((self.x, self.y), self.dir, grid);
            false
        } else {
            self.take_step(grid);
            true
        }
    }

    fn take_step(&mut self, grid: &mut Grid) {
        (self.replace)((self.x, self.y), self.dir, grid);
        let (nx, ny) = self.next_pos();
        self.x = nx;
        self.y = ny;
    }

    fn get_dir(c: char) -> Dir {
        if c == '<' {
            (-1, 0)
        } else if c == '^' {
            (0, -1)
        } else if c == '>' {
            (1, 0)
        } else {
            (0, 1)
        }
    }

    fn rotate(&mut self) {
        self.dir = (-self.dir.1, self.dir.0);
    }

    fn next_pos(&self) -> Pos {
        let (dx, dy) = self.dir;
        (self.x + dx, self.y + dy)
    }

    fn hit_obstacle(&self, grid: &Grid) -> bool {
        let (nx, ny) = self.next_pos();
        if pos_in_bounds(nx, ny, grid) {
            let nx = nx as usize;
            let ny = ny as usize;
            grid[ny][nx] == '#'
        } else {
            false
        }
    }

    fn hit_wall(&self, grid: &Grid) -> bool {
        let (nx, ny) = self.next_pos();
        !pos_in_bounds(nx, ny, grid)
    }

    fn get_guard_pos(grid: &Grid) -> Pos {
        *grid
            .iter()
            .enumerate()
            .filter_map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .filter(|(_, c)| is_guard(c))
                    .map(|(x, _)| (x as isize, y as isize))
                    .collect::<Vec<(isize, isize)>>()
                    .first()
                    .copied()
            })
            .collect::<Vec<(isize, isize)>>()
            .first()
            .unwrap()
    }
    fn get_symbol(dir: Dir) -> char {
        match dir {
            (-1, 0) => '<',
            (0, -1) => '^',
            (1, 0) => '>',
            (0, 1) => 'v',
            _ => {
                let s = format!("Unexpected dir: {} {}", dir.0, dir.1);
                eprintln!("{}", s);
                panic!()
            }
        }
    }
}
fn is_guard(c: &char) -> bool {
    c == &'<' || c == &'^' || c == &'>' || c == &'v'
}

fn pos_in_bounds(x: isize, y: isize, grid: &Grid) -> bool {
    y >= 0 && (y as usize) < grid.len() && x >= 0 && (x as usize) < grid[y as usize].len()
}

fn part1(input: &Vec<String>) -> usize {
    let mut grid = to_matrix(input);
    let replace = |pos: Pos, _dir: Dir, grid: &mut Grid| {
        grid[pos.1 as usize][pos.0 as usize] = 'X';
    };
    let mut guard = Guard::new(&grid, replace);
    while guard.step(&mut grid) {
        let mut grid = grid.clone();
        grid[guard.y as usize][guard.x as usize] =
            Guard::<fn(Pos, Dir, &mut Grid)>::get_symbol(guard.dir);
    }
    grid.iter()
        .map(|v| v.iter().filter(|c| c == &&'X').count())
        .sum()
}

fn part2(input: &Vec<String>) -> usize {
    let mut grid = to_matrix(&input.clone());
    let get_trace = |dir: Dir| match dir {
        (1, 0) => '>',
        (-1, 0) => '<',
        (0, 1) => 'v',
        (0, -1) => '^',
        _ => panic!("Unexpected direction"),
    };
    let perpendicular = |existing: char, trace: char| {
        if ((existing == '^' || existing == 'v') && (trace == '<' || trace == '>'))
            || ((trace == '^' || trace == 'v') && (existing == '<' || existing == '>'))
        {
            true
        } else {
            false
        }
    };
    let replace = |pos: Pos, dir: Dir, grid: &mut Grid| {
        let existing = grid[pos.1 as usize][pos.0 as usize];
        let trace = get_trace(dir);
        if perpendicular(existing, trace) {
            grid[pos.1 as usize][pos.0 as usize] = '+';
        } else {
            grid[pos.1 as usize][pos.0 as usize] = trace
        }
    };
    let mut guard = Guard::new(&grid, replace);
    let initial_pos = (guard.x, guard.y);
    let initial_dir = guard.dir;
    let mut visited = HashSet::<(isize, isize)>::new();
    while guard.step(&mut grid) {
        visited.insert((guard.x, guard.y));
    }

    visited
        .iter()
        .map(|(x, y)| {
            let mut grid = to_matrix(&input.clone());
            grid[*y as usize][*x as usize] = '#';
            let mut guard = Guard::from(initial_pos.0, initial_pos.1, initial_dir, replace);
            let mut visited = HashSet::<(isize, isize, Dir)>::new();
            while guard.step(&mut grid) {
                let state = (guard.x, guard.y, guard.dir);
                if visited.contains(&state) {
                    return true;
                } else {
                    visited.insert(state);
                }
            }
            false
        })
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::util::file_io::get_test_input;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = get_test_input(6);
        let res = part1(&input);
        assert_eq!(res, 41);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(6);
        let res = part2(&input);
        assert_eq!(res, 6);
    }
}
