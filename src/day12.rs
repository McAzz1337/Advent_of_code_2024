use std::{char, collections::HashMap, isize, usize};

use crate::{
    PartFn,
    puzzle_result::PuzzleResult,
    util::util::{Grid, to_matrix},
};

pub fn day12() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    PuzzleResult::new(12, Some(part1), None)
}

type Pos = (usize, usize);

fn adjacent(a: (char, Pos), b: (char, Pos)) -> bool {
    if a.0 == b.0 {
        let (ax, ay) = a.1;
        let (bx, by) = b.1;
        let h = (bx as isize - ax as isize).abs() == 1 && by == ay;
        let v = (by as isize - ay as isize).abs() == 1 && bx == ax;
        (h || v) && !(h && v)
    } else {
        false
    }
}

fn get_regions(grid: &Grid) -> HashMap<char, Vec<Vec<Pos>>> {
    let mapped: Vec<(char, Pos)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .map(|(x, c)| (*c, (x, y)))
                .collect::<Vec<(char, Pos)>>()
        })
        .collect();

    let mut res = HashMap::<char, Vec<Vec<Pos>>>::new();
    mapped.iter().for_each(|(c, pos)| {
        if let Some(v) = res.get_mut(c) {
            if let Some(v1) = v
                .iter_mut()
                .find(|v| v.iter().any(|p| adjacent((*c, *pos), (*c, *p))))
            {
                v1.push(*pos);
            } else {
                v.push(vec![*pos]);
            }
        } else {
            res.insert(*c, vec![vec![*pos]]);
        }
    });

    for _ in 0..10 {
        res = res.iter().map(|r| (*r.0, regroup(*r.0, r.1))).collect();
    }

    res
}

fn regroup(c: char, regions: &Vec<Vec<Pos>>) -> Vec<Vec<(usize, usize)>> {
    let mut res = vec![];
    regions.iter().for_each(|v| {
        if let Some(region) = res.iter_mut().find(|r| find_adjecent(c, v, r)) {
            region.extend(v.iter());
        } else {
            res.push(v.clone());
        }
    });
    res
}

fn find_adjecent(c: char, a: &Vec<Pos>, b: &Vec<Pos>) -> bool {
    a.iter()
        .any(|a_pos| b.iter().any(|b_pos| adjacent((c, *a_pos), (c, *b_pos))))
}

fn count_perimeter(c: char, pos: &Pos, grid: &Grid) -> usize {
    let (x, y) = pos;
    let check_x = |delta: isize| {
        let x = *x as isize + delta;
        (x < 0 || x as usize == grid[*y].len()) || grid[*y][x as usize] != c
    };
    let check_y = |delta: isize| {
        let y = *y as isize + delta;
        (y < 0 || y as usize == grid.len()) || grid[y as usize][*x] != c
    };
    let mut count = 0;
    if check_y(-1) {
        count += 1;
    }
    if check_y(1) {
        count += 1;
    }
    if check_x(-1) {
        count += 1;
    }
    if check_x(1) {
        count += 1;
    }
    count
}

fn get_perimeter(c: char, region: &Vec<Pos>, grid: &Grid) -> usize {
    region
        .iter()
        .map(|(x, y)| count_perimeter(c, &(*x, *y), grid))
        .sum::<usize>()
}

fn get_area(region: &Vec<Pos>) -> usize {
    region.len()
}

fn part1(input: &Vec<String>) -> usize {
    let grid = to_matrix(input);
    let regions = get_regions(&grid);
    regions
        .iter()
        .map(|(c, v)| {
            v.iter()
                .map(|v| {
                    let perimeter = get_perimeter(*c, v, &grid);
                    let area = get_area(v);
                    perimeter * area
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn count_sides(c: char, region: &Vec<Pos>) -> usize {
    unimplemented!()
}

fn part2(input: &Vec<String>) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::{
        day12::{adjacent, regroup},
        util::file_io::get_test_input,
    };

    use super::{Pos, count_perimeter, get_area, get_regions, part1};

    #[test]
    fn test_part1() {
        let input = get_test_input(12);
        let res = part1(&input);
        assert_eq!(res, 1930);
    }

    #[test]
    fn test_adjacent() {
        let a = ('a', (1, 1));
        let b = ('a', (0, 1));
        assert!(adjacent(a, b));

        let a = ('a', (1, 1));
        let b = ('a', (2, 1));
        assert!(adjacent(a, b));

        let a = ('a', (1, 1));
        let b = ('a', (1, 0));
        assert!(adjacent(a, b));

        let a = ('a', (1, 1));
        let b = ('a', (1, 2));
        assert!(adjacent(a, b));
    }

    #[test]
    fn test_count_perimeter() {
        let grid = vec![
            vec!['A', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
        ];

        let res = count_perimeter('A', &(0, 0), &grid);
        assert_eq!(res, 4);

        let grid = vec![
            vec!['A', 'A', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
        ];

        let mut res = count_perimeter('A', &(0, 0), &grid);
        res += count_perimeter('A', &(1, 0), &grid);
        assert_eq!(res, 6);

        let grid = vec![
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['A', 'B', 'A', 'B', 'B', 'B'],
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
        ];

        let mut res = count_perimeter('A', &(0, 0), &grid);
        res += count_perimeter('A', &(1, 0), &grid);
        res += count_perimeter('A', &(2, 0), &grid);
        res += count_perimeter('A', &(2, 1), &grid);
        res += count_perimeter('A', &(2, 2), &grid);
        res += count_perimeter('A', &(1, 2), &grid);
        res += count_perimeter('A', &(0, 2), &grid);
        res += count_perimeter('A', &(0, 1), &grid);
        assert_eq!(res, 16);
    }

    #[test]
    fn test_group() {
        let grid = vec![
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['A', 'C', 'A', 'B', 'B', 'B'],
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'A', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'A'],
        ];

        let regions = get_regions(&grid);
        assert_eq!(regions.get(&'A').unwrap().len(), 3);
        assert_eq!(regions.get(&'B').unwrap().len(), 1);
        assert_eq!(regions.get(&'C').unwrap().len(), 1);
    }

    #[test]
    fn test_get_area() {
        let grid = vec![
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['A', 'C', 'A', 'B', 'B', 'B'],
            vec!['A', 'A', 'A', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'A', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'B'],
            vec!['B', 'B', 'B', 'B', 'B', 'A'],
        ];

        let regions = get_regions(&grid);
        let region = regions.get(&'A').unwrap();
        let r: Vec<Pos> = region.iter().find(|v| v.len() > 1).unwrap().clone();
        let res = get_area(&r);
        assert_eq!(res, 8);

        let region = regions.get(&'C').unwrap();
        let res = get_area(region.get(0).unwrap());
        assert_eq!(res, 1);
    }
}
