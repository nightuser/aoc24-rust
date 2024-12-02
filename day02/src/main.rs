use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Parser;
use itertools::Itertools;

fn is_safe_distance(diff: i32) -> bool {
    (1..=3).contains(&diff.abs())
}

/// `Diff` is a wrapper around the difference.
/// To simplify computations, we support special boundary terminals.
#[derive(Clone, Copy, Debug)]
enum Diff {
    Terminal,
    Num(i32),
}

impl Diff {
    /// Two diffs are compatible if either one of them is a terminal or they have the same sign and
    /// have the same range.
    fn is_compatible(&self, other: &Self) -> bool {
        if let (Self::Num(x), Self::Num(y)) = (self, other) {
            x.signum() == y.signum() && is_safe_distance(*x) && is_safe_distance(*y)
        } else {
            true
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal)
    }

    /// Combines two diffs together. If one of the arguments is a terminal, then the result is also
    /// a terminal. Otherwise, we take the of sum the inner values.
    fn combine(&self, other: &Self) -> Self {
        if let (Self::Num(x), Self::Num(y)) = (self, other) {
            Self::Num(x + y)
        } else {
            Self::Terminal
        }
    }
}

#[derive(Debug, PartialEq)]
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
    if report.len() <= 1 {
        return Safety::Safe;
    }

    // Safety: since the size of `report` is a least 1, unwrap is safe.
    let mut iter = report.iter();
    let first = iter.next().unwrap();
    let mut diffs = iter
        .scan(first, |prev, cur| {
            let diff = cur - *prev;
            *prev = cur;
            Some(diff)
        })
        .map(Diff::Num)
        .chain(iter::once(Diff::Terminal));

    // Safety: since the size of `report` is a least 2, the size of `diffs` is at least 1, and thus
    // unwrap is safe.
    let first = diffs.next().unwrap();

    // We iterate throuth `diffs` with a window of length 3: (prev, cur, next).
    // When we encounter
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
                    if prev.is_terminal() && safety == Safety::Safe {
                        new_branches.push((prev, next, Safety::AlmostSafe));
                    }
                    new_branches.push((cur, next, safety));
                } else {
                    if safety != Safety::Safe {
                        continue;
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
            if branches.into_iter().map(|x| x.2).contains(&Safety::Safe) {
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
