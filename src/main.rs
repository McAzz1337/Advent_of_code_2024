pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod puzzle_result;
pub mod util;

use std::{
    thread::{self, JoinHandle},
    usize,
};

use day1::day1;
use day2::day2;
use day3::day3;
use day4::day4;
use day5::day5;
use day6::day6;
use day7::day7;
use day8::day8;
use day9::day9;
use puzzle_result::PuzzleResult;

pub type PartFn = fn(&Vec<String>) -> usize;

fn main() {
    let functions = vec![day1, day2, day3, day4, day5, day6, day7, day8, day9];

    thread::spawn(move || {
        print!("thread");
    });

    let threads: Vec<(
        usize,
        JoinHandle<PuzzleResult<PartFn, PartFn, usize, usize>>,
    )> = functions
        .into_iter()
        .enumerate()
        .map(|(i, f)| (i, thread::spawn(move || f())))
        .collect();

    let mut results: Vec<(
        usize,
        Result<PuzzleResult<PartFn, PartFn, usize, usize>, Box<dyn std::any::Any + Send>>,
    )> = threads
        .into_iter()
        .map(|(i, t)| match t.join() {
            Ok(result) => (i, Ok(result)),
            Err(e) => (i, Err(e)),
        })
        .collect();

    results.sort_by(|(i, _), (j, _)| i.cmp(j));

    println!();
    results.iter().for_each(|(i, r)| match r {
        Ok(r) => println!("{r}"),
        Err(e) => println!("DAY{i} ERROR: \n{:#?}", e),
    });
}
