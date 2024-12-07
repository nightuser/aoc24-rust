use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn combine_numbers(mut lhs: i64, rhs: i64) -> i64 {
    let mut tmp = rhs;
    while tmp > 0 {
        lhs *= 10;
        tmp /= 10;
    }
    lhs + rhs
}

fn process(target: i64, xs: Vec<i64>) -> (bool, bool) {
    let mut iter = xs.into_iter();
    let first = iter.next().unwrap();
    let mut states1 = HashSet::from([first]);
    let mut states2 = HashSet::from([first]);
    for x in iter {
        let mut new_states1: HashSet<i64> = HashSet::new();
        let mut new_states2: HashSet<i64> = HashSet::new();
        for state in states1 {
            for y in [state + x, state * x] {
                if y <= target {
                    new_states1.insert(y);
                }
            }
        }
        for state in states2 {
            for y in [state + x, state * x, combine_numbers(state, x)] {
                if y <= target {
                    new_states2.insert(y);
                }
            }
        }
        states1 = new_states1;
        states2 = new_states2;
    }
    (states1.contains(&target), states2.contains(&target))
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (target, xs) = line.splitn(2, ": ").collect_tuple().unwrap();
        let target: i64 = target.parse().unwrap();
        let xs: Vec<i64> = xs.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (valid1, valid2) = process(target, xs);
        if valid1 {
            ans1 += target;
        }
        if valid2 {
            ans2 += target;
        }
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
}
