use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn dfs(line: &[u8], pos: usize, towels: &Vec<Box<[u8]>>, count: &mut Vec<i64>) {
    assert!(pos < line.len());
    let suf = &line[pos..];
    let mut total = 0;
    let start = towels.partition_point(|towel| towel[0] < suf[0]);
    for towel in &towels[start..] {
        // Computes the length of the longest common prefix.
        // For some reason this is faster than checking `suf[0] != towel[0]` to break from the loop
        // and then using `suf.starts_with(towel)`.
        let common = suf
            .iter()
            .zip(towel.iter())
            .take_while(|(x, y)| *x == *y)
            .count();
        if common == 0 {
            break;
        }
        if common == towel.len() {
            let new_pos = pos + towel.len();
            if count[new_pos] == -1 {
                dfs(line, new_pos, towels, count);
            }
            total += count[new_pos]
        }
    }
    count[pos] = total;
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let mut reader = BufReader::new(File::open(input).unwrap());

    let mut towels_str = String::new();
    reader.read_line(&mut towels_str).unwrap();
    let mut towels: Vec<Box<[u8]>> = towels_str
        .trim_end()
        .split(", ")
        .map(|s| s.bytes().collect())
        .collect();
    towels.sort(); // Use a radix tree?

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut count: Vec<i64> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let line = line.into_bytes().into_boxed_slice();
        let len = line.len();
        count.resize(len + 1, 0);
        count.fill(-1);
        count[len] = 1;
        dfs(&line, 0, &towels, &mut count);
        let total = count[0];
        if total > 0 {
            ans1 += 1;
            ans2 += total;
        }
    }
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
