use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
    vec,
};

use crate::{
    PartFn,
    puzzle_result::PuzzleResult,
    util::util::{Grid, to_matrix},
};

pub fn day8() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    PuzzleResult::new(8, Some(part1), Some(part2))
}

type Pos = (usize, usize);
type Group = (char, Vec<Pos>);

#[derive(Clone, PartialEq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    fn from(pos: &Pos) -> Vec2 {
        Vec2 {
            x: pos.0 as isize,
            y: pos.1 as isize,
        }
    }

    fn to_pos(&self) -> Option<Pos> {
        if self.x >= 0 && self.y >= 0 {
            Some((self.x as usize, self.y as usize))
        } else {
            None
        }
    }

    fn mul(&self, factor: isize) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("[{}, {}]", self.x, self.y))
    }
}

fn group(antenna: &char, grid: &Grid) -> Group {
    let filter_map = |(y, v): (usize, &Vec<char>)| {
        v.iter()
            .enumerate()
            .filter(|(_, c)| c == &antenna)
            .map(|(x, _)| (x, y))
            .collect::<Vec<Pos>>()
    };
    let group: Vec<Pos> = grid.iter().enumerate().flat_map(filter_map).collect();

    (*antenna, group)
}

fn create_antinode(group: &Group, grid: &Grid) -> Vec<Pos> {
    let (antenna, positions) = group;
    positions
        .iter()
        .flat_map(|pos| {
            let others: Vec<&Pos> = positions.iter().filter(|p| p != &pos).collect();
            others
                .iter()
                .map(|p| Vec2::from(p) - Vec2::from(pos))
                .map(|v| Vec2::from(pos) + v.mul(2))
                .filter(|v| vec_in_bounds(v, grid))
                .filter_map(|v| v.to_pos())
                .collect::<Vec<Pos>>()
        })
        .filter(|v| &grid[v.1][v.0] != antenna)
        .collect()
}

fn vec_in_bounds(v: &Vec2, grid: &Grid) -> bool {
    if let Some(pos) = v.to_pos() {
        pos_in_bounds(&pos, grid)
    } else {
        false
    }
}

fn pos_in_bounds(pos: &Pos, grid: &Grid) -> bool {
    let (x, y) = pos;
    *y < grid.len() && *x < grid[*y].len()
}

fn part1(input: &Vec<String>) -> usize {
    let map = to_matrix(input);
    let antennas: Vec<char> = map
        .iter()
        .flat_map(|v| {
            v.iter()
                .filter(|c| c != &&'.')
                .copied()
                .collect::<Vec<char>>()
        })
        .collect();
    let groups: HashSet<Group> = antennas.iter().map(|c| group(c, &map)).collect();
    groups
        .iter()
        .flat_map(|g| create_antinode(g, &map))
        .collect::<HashSet<Pos>>()
        .len()
}

fn continouous(origin: Vec2, direction: Vec2, grid: &Grid) -> Vec<Pos> {
    let mut res = vec![];
    let mut factor = 1;
    let mut antenna = origin.clone() + direction.mul(factor);
    while vec_in_bounds(&antenna, grid) {
        res.push(antenna.to_pos().unwrap());
        factor += 1;
        antenna = origin.clone() + direction.mul(factor);
    }
    res
}

fn create_antinode2(group: &Group, grid: &Grid) -> Vec<Pos> {
    let (_, positions) = group;
    positions
        .iter()
        .flat_map(|pos| {
            let others: Vec<&Pos> = positions.iter().filter(|p| p != &pos).collect();
            let res = others
                .iter()
                .map(|p| Vec2::from(p) - Vec2::from(pos))
                .flat_map(|v| continouous(Vec2::from(pos), v, grid))
                .collect::<Vec<Pos>>();
            [res, vec![*pos]].concat()
        })
        // .filter(|v| &grid[v.1][v.0] != antenna)
        .collect()
}

fn part2(input: &Vec<String>) -> usize {
    let map = to_matrix(input);
    let antennas: Vec<char> = map
        .iter()
        .flat_map(|v| {
            v.iter()
                .filter(|c| c != &&'.')
                .copied()
                .collect::<Vec<char>>()
        })
        .collect();
    let groups: HashSet<Group> = antennas.iter().map(|c| group(c, &map)).collect();
    groups
        .iter()
        .flat_map(|g| create_antinode2(g, &map))
        .collect::<HashSet<Pos>>()
        .len()
}
#[cfg(test)]
mod tests {
    use crate::util::{file_io::get_test_input, util::to_matrix};

    use super::{create_antinode, group, part1, part2};

    #[test]
    fn test_part1() {
        let input = get_test_input(8);
        let res = part1(&input);
        assert_eq!(res, 14);
    }

    #[test]
    fn test_group() {
        let input = [
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let grid = to_matrix(&input);

        let res = group(&'0', &grid);
        assert_eq!(res.1.len(), 4);
        let res = group(&'A', &grid);
        assert_eq!(res.1.len(), 3);
    }

    #[test]
    fn test_create_antinode() {
        let input = [
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        let grid = to_matrix(&input);
        let g = group(&'0', &grid);
        let res = create_antinode(&g, &grid);
        assert_eq!(res.len(), 10);
        let g = group(&'A', &grid);
        let res = create_antinode(&g, &grid);
        println!("res: {:#?}", res);
        assert_eq!(res.len(), 5);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(8);
        let res = part2(&input);
        assert_eq!(res, 34);
    }
}
