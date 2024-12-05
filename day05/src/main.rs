use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = env::args_os().nth(1).context("specify input file")?;
    let mut reader = BufReader::new(File::open(input)?);
    let mut reqs: HashSet<(i32, i32)> = HashSet::new();
    for line in (&mut reader).lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let req: (i32, i32) = line
            .split('|')
            .filter_map(|s| s.parse().ok())
            .collect_tuple()
            .context("incorrect format")?;
        reqs.insert(req);
    }
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in reader.lines() {
        let line = line?;
        let mut pages = line
            .split(',')
            .map(|s| s.parse())
            .collect::<Result<Vec<i32>, _>>()?;
        let mut valid = true;
        let len = pages.len();
        for i in 0..len - 1 {
            let mut restart = true;
            while restart {
                restart = false;
                let current = pages[i];
                for j in i + 1..len {
                    let candidate = pages[j];
                    if reqs.contains(&(candidate, current)) {
                        valid = false;
                        restart = true;
                        pages.swap(i, j);
                        break;
                    }
                }
            }
        }
        let middle = pages[pages.len() / 2];
        if valid {
            ans1 += middle;
        } else {
            ans2 += middle;
        }
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
    Ok(())
}
