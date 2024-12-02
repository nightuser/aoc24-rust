use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Parser;

const fn is_safe_distance(diff: i32) -> bool {
    matches!(diff.abs(), 1..=3)
}

/// `Diff` is a wrapper around the difference.
/// To simplify computations, we support special boundary terminals.
#[derive(Clone, Copy, Debug)]
enum Diff {
    Terminal,
    Num(i32),
}

impl Diff {
    /// Two diffs are compatible if either one of them is a terminal or they have the same sign
    /// and have the same range.
    const fn is_compatible(&self, other: &Self) -> bool {
        if let (Self::Num(x), Self::Num(y)) = (self, other) {
            x.signum() == y.signum() && is_safe_distance(*x) && is_safe_distance(*y)
        } else {
            true
        }
    }

    /// Combines two diffs together. If one of the arguments is a terminal, then the result is
    /// also a terminal. Otherwise, we take the of sum the inner values.
    const fn combine(&self, other: &Self) -> Self {
        if let (Self::Num(x), Self::Num(y)) = (self, other) {
            Self::Num(*x + *y)
        } else {
            Self::Terminal
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Safety {
    Safe,
    AlmostSafe,
    Unsafe,
}

#[derive(Parser)]
struct Cli {
    input: PathBuf,
}

fn get_safety(report: &[i32]) -> Safety {
    // `diffs` is an iterator over consecutive differences, terminated by a terminal.
    let mut diffs = report
        .windows(2)
        .map(|w| Diff::Num(w[1] - w[0]))
        .chain(iter::once(Diff::Terminal));

    // If `diffs` is empty, the size of `report` is less than 2 and it's always safe.
    let first = match diffs.next() {
        Some(first) => first,
        None => return Safety::Safe,
    };

    // We iterate throuth `diffs` with a window of length 3: (prev, cur, next).
    let branches = diffs.try_fold(
        vec![(Diff::Terminal, first, Safety::Safe)],
        |branches, next| {
            // There is at most three branches:
            // * We dropped the first element.
            // * We combined two consective elements together in one of two ways.
            // Such a split can occur at most once.
            assert!(branches.len() <= 3);
            let mut new_branches: Vec<(Diff, Diff, Safety)> = Vec::with_capacity(3);
            for (prev, cur, safety) in branches {
                if prev.is_compatible(&cur) {
                    // Corner case: we might need to drop the first element.
                    if let (Safety::Safe, Diff::Terminal) = (&safety, prev) {
                        new_branches.push((prev, next, Safety::AlmostSafe))
                    }
                    new_branches.push((cur, next, safety));
                } else {
                    match safety {
                        Safety::Safe => {}
                        _ => continue,
                    }
                    let prev_cur = prev.combine(&cur);
                    if prev_cur.is_compatible(&next) {
                        new_branches.push((prev_cur, next, Safety::AlmostSafe));
                    }
                    let cur_next = cur.combine(&next);
                    if prev.is_compatible(&cur_next) {
                        new_branches.push((prev, cur_next, Safety::AlmostSafe));
                    }
                }
            }
            if new_branches.is_empty() {
                None
            } else {
                Some(new_branches)
            }
        },
    );

    match branches {
        Some(branches) => {
            if branches
                .into_iter()
                .map(|b| b.2)
                .any(|s| matches!(s, Safety::Safe))
            {
                Safety::Safe
            } else {
                Safety::AlmostSafe
            }
        }
        None => Safety::Unsafe,
    }
}

fn parse_line<T: FromStr>(line: String) -> Result<Vec<T>, T::Err> {
    line.split_whitespace().map(|x| x.parse()).collect()
}

fn run(input: &Path) -> anyhow::Result<()> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut safe = 0;
    let mut almost_safe = 0;
    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = parse_line(line)?;
        match get_safety(&report) {
            Safety::Safe => safe += 1,
            Safety::AlmostSafe => almost_safe += 1,
            _ => {}
        }
    }

    println!("ans1 = {}", safe);
    println!("ans2 = {}", safe + almost_safe);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    run(&args.input)
}
