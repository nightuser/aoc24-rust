use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MASK: i32 = 0xfffff;

fn next_secret_number(mut x: u64) -> u64 {
    x = ((x * 64) ^ x) % 16777216;
    x = ((x / 32) ^ x) % 16777216;
    x = ((x * 2048) ^ x) % 16777216;
    x
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());

    let mut ans1: u64 = 0;
    let mut storage: Box<[i32]> = vec![0; 1 << 20].into_boxed_slice();
    let mut seen: Box<[bool]> = vec![false; 1 << 20].into_boxed_slice();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut input: u64 = line.parse().unwrap();
        seen.fill(false);
        let mut value = 0;
        let mut prev = (input % 10) as i32;

        for _ in 0..3 {
            input = next_secret_number(input);
            let current = (input % 10) as i32;
            let diff = (current - prev) + 10;
            value = (value << 5) | diff;
            prev = current;
        }

        for _ in 0..1997 {
            input = next_secret_number(input);
            let current = (input % 10) as i32;
            let diff = (current - prev) + 10;
            value = ((value << 5) & MASK) | diff;
            let value_index = value as usize;
            if !seen[value_index] {
                storage[value_index] += current;
            }
            seen[value_index] = true;
            prev = current;
        }
        ans1 += input;
    }
    let ans2 = IntoIterator::into_iter(storage).max().unwrap();
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
