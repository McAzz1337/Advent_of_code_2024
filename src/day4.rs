use crate::{
    PartFn,
    puzzle_result::PuzzleResult,
    util::util::{Grid, to_matrix},
};

pub fn day4() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    PuzzleResult::new(4, Some(part1), Some(part2))
}

type Pos = (usize, usize);
type PosOption = (Option<usize>, Option<usize>);

fn check<F>(mat: &Grid, term: &str, pos: Pos, step: F) -> Option<String>
where
    F: Fn(usize, usize) -> PosOption,
{
    let (mut x, mut y) = pos;
    let bound_check = |x: usize, y: usize| y < mat.len() && x < mat[y].len();
    let advance = |x: usize, y: usize| match step(x, y) {
        (Some(new_x), Some(new_y)) => {
            if bound_check(new_x, new_y) {
                Some((new_x, new_y))
            } else {
                None
            }
        }
        _ => None,
    };
    let mut s = String::new();
    for _ in 0..term.len() {
        s.push(mat[y][x]);
        if let Some((new_x, new_y)) = advance(x, y) {
            x = new_x;
            y = new_y;
        } else {
            break;
        }
    }
    Some(s)
}

fn check_pos(mat: &Grid, term: &str, pos: Pos) -> usize {
    let steps = [
        |x: usize, y: usize| (Some(x + 1), Some(y)), // Right
        |x: usize, y: usize| (Some(x + 1), y.checked_sub(1)), // Top-right diagonal
        |x: usize, y: usize| (Some(x + 1), Some(y + 1)), // Bottom-right diagonal
        |x: usize, y: usize| (x.checked_sub(1), Some(y)), // Left
        |x: usize, y: usize| (x.checked_sub(1), y.checked_sub(1)), // Top-left diagonal
        |x: usize, y: usize| (x.checked_sub(1), Some(y + 1)), // Bottom-left diagonal
        |x: usize, y: usize| (Some(x), Some(y + 1)), // Down
        |x: usize, y: usize| (Some(x), y.checked_sub(1)), // Up
    ];

    let mapper = |step: &fn(usize, usize) -> PosOption| check(mat, term, pos, step);
    let predicate = |s: &String| s == term;

    steps.iter().filter_map(mapper).filter(predicate).count()
}

fn get_positions(mat: &Grid) -> Vec<Pos> {
    mat.iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'X')
                .map(|(x, _)| (x, y))
                .collect::<Vec<Pos>>()
        })
        .collect()
}

fn part1(input: &Vec<String>) -> usize {
    let mat = &to_matrix(input);
    get_positions(mat)
        .iter()
        .map(|pos| check_pos(mat, "XMAS", *pos))
        .sum()
}

fn other_char_of(m: char, c: char) -> Option<char> {
    if m == 'M' && c == 'S' {
        Some('S')
    } else if m == 'S' && c == 'M' {
        Some('M')
    } else {
        None
    }
}

fn is_part_of_x(c: char) -> Option<char> {
    if c == 'M' || c == 'S' { Some(c) } else { None }
}

fn check_x(mat: &Grid, pos: &Pos) -> bool {
    let (x, y) = *pos;
    if y > 0 && y < mat.len() - 1 && x > 0 && x < mat[y].len() - 1 {
        if let Some(ul) = is_part_of_x(mat[y - 1][x - 1]) {
            if other_char_of(ul, mat[y + 1][x + 1]).is_some() {
                if let Some(ur) = is_part_of_x(mat[y - 1][x + 1]) {
                    if other_char_of(ur, mat[y + 1][x - 1]).is_some() {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn get_potential_centers(mat: &Grid) -> Vec<Pos> {
    mat.iter()
        .enumerate()
        .flat_map(|(y, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, c)| c == &&'A')
                .map(|(x, _)| (x, y))
                .collect::<Vec<Pos>>()
        })
        .collect()
}

fn part2(input: &Vec<String>) -> usize {
    let mat = to_matrix(input);
    let positions = get_potential_centers(&mat);
    positions
        .iter()
        .map(|pos| check_x(&mat, pos))
        .filter(|b| *b)
        .count()
}

#[cfg(test)]
mod tests {

    use crate::util::file_io::get_test_input_part;

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = get_test_input_part(4, 1);
        let res = part1(&input);
        assert_eq!(res, 18);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input_part(4, 2);
        let res = part2(&input);
        assert_eq!(res, 9);
    }
}
