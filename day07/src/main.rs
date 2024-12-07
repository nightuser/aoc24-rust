use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use itertools::Itertools;
use rustc_hash::FxHashSet;

fn combine_numbers(mut lhs: i64, rhs: i64) -> i64 {
    let mut tmp = rhs;
    while tmp > 0 {
        lhs *= 10;
        tmp /= 10;
    }
    lhs + rhs
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut states = FxHashSet::default();
    let mut new_states = FxHashSet::default();
    for line in reader.lines() {
        let line = line.unwrap();
        let (target, xs) = line.splitn(2, ": ").collect_tuple().unwrap();
        let target = target.parse().unwrap();
        let mut xs = xs.split_whitespace().map(|s| s.parse().unwrap());

        let first = xs.next().unwrap();
        states.insert((first, true));

        for x in xs {
            for (state, state_simple) in states.drain() {
                for (y, y_simple) in [
                    (state + x, true),
                    (state * x, true),
                    (combine_numbers(state, x), false),
                ] {
                    if y <= target {
                        new_states.insert((y, state_simple && y_simple));
                    }
                }
            }
            mem::swap(&mut states, &mut new_states);
        }
        if states.contains(&(target, true)) {
            ans1 += target;
            ans2 += target;
        } else if states.contains(&(target, false)) {
            ans2 += target;
        }
        states.clear();
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
}
