use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::mem;

const NUM_LINES: i32 = 7;

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut width = 0;
    let mut is_lock = true;
    let mut locks: Vec<Vec<i32>> = Vec::new();
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut cur_obj: Vec<i32> = Vec::new();
    for (line, line_no) in reader
        .lines()
        .chain(iter::once(Ok(String::new())))
        .zip((0..=NUM_LINES).cycle())
    {
        let line = line.unwrap().into_bytes();
        if line_no == NUM_LINES {
            assert!(line.is_empty());
            if is_lock { &mut locks } else { &mut keys }
                .push(mem::replace(&mut cur_obj, vec![0; width]));
            continue;
        }
        if line_no == 0 {
            if width == 0 {
                width = line.len();
                cur_obj.resize(width, 0);
            }
            is_lock = line[0] == b'#';
        }
        for (c, out) in line.into_iter().zip(cur_obj.iter_mut()) {
            if c == b'#' {
                *out += 1;
            }
        }
    }
    let mut ans = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock
                .iter()
                .zip(key)
                .map(|(x, y)| x + y)
                .all(|h| h <= NUM_LINES)
            {
                ans += 1;
            }
        }
    }
    println!("ans = {ans}");
}
