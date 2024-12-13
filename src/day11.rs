use std::{
    sync::{Arc, Mutex},
    thread::{self},
    usize,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{PartFn, puzzle_result::PuzzleResult};

pub fn day11() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    PuzzleResult::omitted(11, Some(part1), Some(part2))
}

fn prepare(input: &[String]) -> Vec<usize> {
    input
        .first()
        .unwrap()
        .split(" ")
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn rule1(x: usize) -> bool {
    x == 0
}

fn rule2(x: usize) -> bool {
    x.to_string().len() & 0b1 == 0
}

fn split_in_two(x: &usize) -> (usize, usize) {
    let s = x.to_string();
    let mid = s.len() / 2;
    let left = &s[..mid];
    let right = &s[mid..];
    (
        left.parse::<usize>().unwrap(),
        right.parse::<usize>().unwrap(),
    )
}

fn transform(x: &usize) -> Vec<usize> {
    if rule1(*x) {
        vec![1]
    } else if rule2(*x) {
        let (left, right) = split_in_two(x);
        vec![left, right]
    } else {
        vec![x * 2024]
    }
}

fn part1(input: &Vec<String>) -> usize {
    let mut input = prepare(input);
    (0..25).for_each(|_| {
        input = input.iter().flat_map(transform).collect();
    });

    input.len()
}

fn fracture(input: &Vec<usize>) -> Vec<Vec<usize>> {
    let length = input.len();
    let chunk_len = length / 7;
    println!("len {length}/ 7 = {chunk_len}");
    let c = input.chunks_exact(chunk_len);
    let mut chunks: Vec<Vec<usize>> = c.clone().map(|c| c.to_vec()).collect();
    chunks.push(c.remainder().to_vec());
    chunks
}

fn part2(input: &Vec<String>) -> usize {
    let mut list = prepare(input);
    // println!(
    //     "initial: {}",
    //     list.iter()
    //         .map(|x| x.to_string())
    //         .fold(String::new(), |a, b| a + b.as_str() + ", ")
    // );
    (0..75).for_each(|j| {
        if j >= 26 {
            // println!(
            //     "from: {}",
            //     list.iter()
            //         .map(|x| x.to_string())
            //         .fold(String::new(), |a, b| a + b.as_str() + ", ")
            // );
            let chunks = fracture(&list);
            assert!(chunks.len() <= 9);
            // chunks.iter().for_each(|c| {
            //     let s = c
            //         .iter()
            //         .map(|x| x.to_string())
            //         .fold(String::new(), |a, b| a + b.as_str() + ", ");
            //     println!("chunk: {s}");
            // });
            let chunks = Arc::new(Mutex::new(chunks));

            let threads: Vec<_> = (0..9)
                .map(|i| {
                    let chunks: Arc<_> = Arc::clone(&chunks);
                    thread::spawn(move || {
                        let chunk = chunks.lock().unwrap().pop();
                        if let Some(c) = chunk {
                            (i, c.par_iter().flat_map(transform).collect::<Vec<usize>>())
                        } else {
                            (i, vec![])
                        }
                    })
                })
                .collect();
            let mut res = vec![];
            for t in threads {
                match t.join() {
                    Ok(r) => res.push(r),
                    Err(e) => println!("error {:#?}", e),
                }
            }
            res.sort_by(|(i, _), (j, _)| i.cmp(j));
            let res = res.iter().rev().flat_map(|(_, v)| v).copied().collect();
            list = res;
            // println!(
            //     "now: {}",
            //     list.iter()
            //         .map(|x| x.to_string())
            //         .fold(String::new(), |a, b| a + b.as_str() + ", ")
            // );
        } else {
            list = list.iter().flat_map(transform).collect();
        }
        // println!("{:#?}", list);
    });

    assert_eq!(list.len(), part1(input));
    list.len()
}

#[cfg(test)]
mod tests {
    use crate::{day11::prepare, util::file_io::get_test_input};

    use super::{part1, transform};

    #[test]
    fn test_part1() {
        let input = get_test_input(11);
        let res = part1(&input);
        assert_eq!(res, 55312);
    }

    #[test]
    fn test_part1_once() {
        let input = get_test_input(11);
        let input = prepare(&input);
        let res: Vec<usize> = input.iter().flat_map(|x| transform(x)).collect();
        assert_eq!(res.len(), 3);
    }
}
