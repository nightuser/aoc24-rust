use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Parser;
use itertools::Itertools;

enum Safety {
    Safe,
    AlmostSafe,
    Unsafe,
}

#[derive(Parser)]
struct Cli {
    input: PathBuf,
}

fn is_safe(report: &[i32]) -> bool {
    match report {
        [] | [_] => true,
        [first, second, ..] => {
            let sign = (second - first).signum();
            report.iter().tuple_windows().all(|(cur, next)| {
                let diff = next - cur;
                diff.signum() == sign && (1..=3).contains(&diff.abs())
            })
        }
    }
}

// Simple quadratic solution.
// Can be done in linear time, but the input is too small to care about complexity.
fn get_safety(report: &[i32]) -> Safety {
    if is_safe(report) {
        return Safety::Safe;
    }
    for i in 0..report.len() {
        let mut candidate = report[..i].to_vec();
        candidate.extend(report[i + 1..].iter());
        if is_safe(&candidate) {
            return Safety::AlmostSafe;
        }
    }
    Safety::Unsafe
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
    let ans1 = safe;
    let ans2 = safe + almost_safe;

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    run(&args.input)
}
