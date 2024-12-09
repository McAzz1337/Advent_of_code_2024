use std::collections::HashMap;

use crate::{PartFn, puzzle_result::PuzzleResult, util::file_io::get_input};

pub fn day5() -> PuzzleResult<PartFn, PartFn, usize, usize> {
    let input = get_input(5);
    PuzzleResult::new(5, input, Some(part1), Some(part2))
}

type Rule = (usize, usize);
type Update = Vec<usize>;
type RuleMap = HashMap<usize, Vec<usize>>;

fn contained(key: &usize, to_check: Vec<&usize>, rules: &RuleMap) -> bool {
    let check = |k: &&usize| rules.get(k).map_or(true, |v| !v.contains(key));
    to_check.iter().all(check)
}

fn is_valid(update: &Update, rules: &RuleMap) -> bool {
    update.iter().enumerate().all(|(i, u)| {
        let to_check: Vec<&usize> = update.iter().skip(i + 1).collect();
        contained(u, to_check, rules)
    })
}

fn filter_valid_updates(rules: RuleMap, updates: Vec<Update>) -> Vec<Update> {
    updates
        .iter()
        .filter(|u| is_valid(u, &rules))
        .cloned()
        .collect()
}

fn split_on(s: &str, token: &str) -> Vec<String> {
    s.split(token).map(|s| s.to_string()).collect()
}

fn vec_to_hashmap(v: Vec<Rule>) -> HashMap<usize, Vec<usize>> {
    let mut res: HashMap<usize, Vec<usize>> = HashMap::new();
    v.iter().for_each(|(k, v)| {
        if let Some(vec) = res.get_mut(k) {
            vec.push(*v);
        } else {
            res.insert(*k, vec![*v]);
        }
    });

    res
}

fn prepare(input: &[String]) -> (Vec<Update>, Vec<Rule>) {
    let (rules, updates): (Vec<String>, Vec<String>) =
        input.iter().cloned().partition(|s| s.contains("|"));

    let split_on_pipe = |s: &String| split_on(s, "|");
    let take_first_two = |v: Vec<String>| (v[0].clone(), v[1].clone());
    let string_pair_to_usize = |p: (String, String)| {
        let (a, b) = p;
        (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
    };
    let rules: Vec<Rule> = rules
        .iter()
        .map(split_on_pipe)
        .map(take_first_two)
        .map(string_pair_to_usize)
        .collect();

    let split_on_comma = |s: &String| split_on(s, ",");
    let vec_str_to_usize = |s: Vec<String>| s.iter().map(|s| s.parse::<usize>().unwrap()).collect();
    let updates: Vec<Update> = updates
        .iter()
        .map(split_on_comma)
        .map(vec_str_to_usize)
        .collect();

    (updates, rules)
}

fn part1(input: &Vec<String>) -> usize {
    let (updates, rules) = prepare(input);
    let rules = vec_to_hashmap(rules);
    let updates = filter_valid_updates(rules, updates);
    updates.iter().map(|v| v[v.len() / 2]).sum()
}

fn out_of_place(key: &usize, to_check: Vec<&usize>, rules: &RuleMap) -> Option<usize> {
    to_check
        .iter()
        .enumerate()
        .find(|(_, k)| rules.get(k).is_some_and(|v| v.contains(key)))
        .map(|x| x.0)
}

fn fix_update(index: &usize, update: &Update, rules: &RuleMap) -> Option<Update> {
    let to_correct: Vec<(usize, usize)> = update
        .iter()
        .enumerate()
        .filter_map(|(i, u)| {
            let to_check: Vec<&usize> = update.iter().take(i).collect();
            out_of_place(u, to_check, &rules).map(|j| (i, j))
        })
        .collect();
    if to_correct.is_empty() {
        Some(update.clone())
    } else {
        to_correct.iter().find_map(|(i, j)| {
            let mut u = update.clone();
            u.swap(*i, *j);
            if !is_valid(&u, rules) {
                fix_update(index, &u, rules)
            } else {
                Some(u)
            }
        })
    }
}

fn part2(input: &Vec<String>) -> usize {
    let (updates, rules) = prepare(input);
    let rules = vec_to_hashmap(rules);
    let valid_updates = filter_valid_updates(rules.clone(), updates.clone());
    let updates: Vec<Update> = updates
        .iter()
        .filter(|u| !valid_updates.contains(u))
        .cloned()
        .collect();
    let updates: Vec<Update> = updates
        .iter()
        .enumerate()
        .filter_map(|(i, u)| fix_update(&i, u, &rules))
        .collect();
    updates.iter().map(|v| v[v.len() / 2]).sum()
}

#[cfg(test)]
mod tests {
    use crate::{
        day5::{is_valid, vec_to_hashmap},
        util::file_io::get_test_input,
    };

    use super::{part1, part2, prepare};

    #[test]
    fn test_part1() {
        let input = get_test_input(5);
        let res = part1(&input);
        assert_eq!(res, 143);
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(5);
        let res = part2(&input);
        assert_eq!(res, 123);
    }

    #[test]
    fn some() {
        let v = vec![97, 75, 47, 29, 13];
        let input = get_test_input(5);
        let (_, rules) = prepare(&input);
        assert!(is_valid(&v, &vec_to_hashmap(rules)))
    }
}
