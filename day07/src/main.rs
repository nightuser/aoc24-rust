use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use itertools::Itertools;
use rustc_hash::FxHashSet;

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
        let target: i64 = target.parse().unwrap();
        let xs = xs.split_whitespace().map(|s| s.parse().unwrap()).rev();

        states.insert((target, true));

        for x in xs {
            let mut e = 1;
            let mut tmp = x;
            while tmp != 0 {
                tmp /= 10;
                e *= 10;
            }
            for (state, state_simple) in states.drain() {
                if state % x == 0 {
                    new_states.insert((state / x, state_simple));
                }
                if state >= x {
                    new_states.insert((state - x, state_simple));
                }
                if state % e == x {
                    new_states.insert((state / e, false));
                }
            }
            mem::swap(&mut states, &mut new_states);
        }
        if states.contains(&(0, true)) {
            ans1 += target;
            ans2 += target;
        } else if states.contains(&(0, false)) {
            ans2 += target;
        }
        states.clear();
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
}
