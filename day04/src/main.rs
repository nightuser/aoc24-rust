use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, iter};

use anyhow::anyhow;

type Point = (usize, usize);
type PointShift = (isize, isize);
type Grid<T> = HashMap<Point, T>;

const SHIFTS: [PointShift; 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

const NEEDLE: &str = "XMAS";

fn shift_point((x, y): Point, (dx, dy): PointShift) -> Point {
    (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))
}

fn get_string(grid: &Grid<char>, indices: impl IntoIterator<Item = Point>) -> Option<String> {
    indices.into_iter().map(|p| grid.get(&p)).collect()
}

fn main() -> anyhow::Result<()> {
    let input = env::args_os()
        .nth(1)
        .ok_or(anyhow!("provide an input file"))?;
    let reader = BufReader::new(File::open(input)?);

    let mut grid: Grid<char> = HashMap::new();
    let valid_symbols: HashSet<_> = NEEDLE.chars().collect();
    let mut x_starts: Vec<Point> = Vec::new();
    let mut a_starts: Vec<Point> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let row: Vec<_> = line.chars().collect();
        for (x, c) in row.into_iter().enumerate() {
            if !valid_symbols.contains(&c) {
                continue;
            }
            match c {
                'X' => x_starts.push((x, y)),
                'A' => a_starts.push((x, y)),
                _ => {}
            };
            grid.insert((x, y), c);
        }
    }

    let ans1: usize = x_starts
        .into_iter()
        .map(|start| {
            SHIFTS
                .into_iter()
                .filter(|&shift| {
                    let indices = iter::successors(Some(start), |&p| Some(shift_point(p, shift)))
                        .take(NEEDLE.len());
                    get_string(&grid, indices).is_some_and(|s| s == NEEDLE)
                })
                .count()
        })
        .sum();
    println!("ans1 = {ans1}");

    let ans2 = a_starts
        .into_iter()
        .filter(|&start| {
            [[(1, 1), (-1, -1)], [(1, -1), (-1, 1)]]
                .into_iter()
                .all(|shifts| -> bool {
                    let indices = shifts.into_iter().map(|shift| shift_point(start, shift));
                    get_string(&grid, indices).is_some_and(|s| s == "MS" || s == "SM")
                })
        })
        .count();
    println!("ans2 = {ans2}");

    Ok(())
}
