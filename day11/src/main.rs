use std::env;
use std::fs;
use std::mem;

use hashbrown::HashMap;
use ilog::IntLog;

const POWERS: [i64; 9] = [
    10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000,
];

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let contents = fs::read_to_string(input).unwrap();
    let mut state: HashMap<i64, i64> = contents
        .split_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    let mut new_state: HashMap<i64, i64> = HashMap::new();
    let mut ans1: i64 = 0;
    for blink in 1..=75 {
        for (x, cnt) in state.drain() {
            if x == 0 {
                *new_state.entry(1).or_default() += cnt;
                continue;
            }
            let log10 = x.log10(); // One less than the nubmer of digits.
            if log10 % 2 == 0 {
                *new_state.entry(x * 2024).or_default() += cnt;
            } else {
                let digits = (log10 - 1) / 2; // Intentionally off by one.
                let exp = POWERS[digits];
                *new_state.entry(x / exp).or_default() += cnt;
                *new_state.entry(x % exp).or_default() += cnt;
            }
        }
        mem::swap(&mut state, &mut new_state);
        if blink == 25 {
            ans1 = state.iter().map(|(_, cnt)| cnt).sum();
        }
    }
    let ans2: i64 = state.iter().map(|(_, cnt)| cnt).sum();
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
