use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::zip;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::bail;
use anyhow::Context;
use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
struct Cli {
    input: PathBuf,
}

fn split_line<T: FromStr>(line: &str) -> Result<Vec<T>, T::Err> {
    line.split_whitespace().map(str::parse).collect()
}

fn parse(input: &Path) -> anyhow::Result<(Vec<i32>, Vec<i32>)> {
    let mut xs: Vec<i32> = Vec::new();
    let mut ys: Vec<i32> = Vec::new();
    let input_file = File::open(input)?;
    let input_reader = BufReader::new(input_file);
    for line in input_reader.lines() {
        let line = line?;
        let parts: Vec<i32> = split_line(&line)?;
        match parts[..] {
            [x, y] => {
                xs.push(x);
                ys.push(y);
            }
            _ => bail!("incorrect file format"),
        }
    }
    Ok((xs, ys))
}

fn run<P: AsRef<Path>>(input_path: P) -> anyhow::Result<()> {
    let input = input_path.as_ref();
    let (mut xs, mut ys) = parse(input).with_context(|| format!("cannot parse {input:?}"))?;
    xs.sort();
    ys.sort();

    let ans1: i32 = zip(&xs, &ys).map(|(x, y)| (x - y).abs()).sum();
    println!("ans1 = {ans1}");

    let counts = ys.into_iter().counts();
    let ans2: i32 = xs
        .iter()
        .map(|x| x * *counts.get(x).unwrap_or(&0) as i32)
        .sum();
    println!("ans2 = {ans2}");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    run(args.input)
}
