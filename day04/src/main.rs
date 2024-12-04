use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, iter};

use anyhow::anyhow;

#[derive(Debug)]
struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        self.data.push(row);
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data.get(y).and_then(|row| row.get(x))
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new()
    }
}

const SHIFTS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn shift_point((x, y): (usize, usize), (dx, dy): (isize, isize)) -> (usize, usize) {
    (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))
}

fn main() -> anyhow::Result<()> {
    let input = env::args_os()
        .nth(1)
        .ok_or(anyhow!("provide an input file"))?;
    let reader = BufReader::new(File::open(input)?);
    let mut grid: Grid<char> = Grid::new();
    let mut x_starts: Vec<(usize, usize)> = Vec::new();
    let mut a_starts: Vec<(usize, usize)> = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line?;
        let row: Vec<_> = line.chars().collect();
        for (x, c) in row.iter().enumerate() {
            match *c {
                'X' => x_starts.push((x, y)),
                'A' => a_starts.push((x, y)),
                _ => {}
            }
        }
        grid.add_row(row);
    }

    let ans1: usize = x_starts
        .into_iter()
        .map(|start| {
            SHIFTS
                .into_iter()
                .filter(|&shift| {
                    let positions = iter::successors(Some(start), |&p| Some(shift_point(p, shift)));
                    "XMAS"
                        .chars()
                        .zip(positions)
                        .all(|(exp_c, (x, y))| grid.get(x, y).is_some_and(|&c| c == exp_c))
                })
                .count()
        })
        .sum();
    println!("ans1 = {ans1}");

    let required_neighbors = HashSet::from(['M', 'S']);
    let ans2 = a_starts
        .into_iter()
        .filter(|&start| {
            let is_valid = |shifts: &[(isize, isize)]| -> bool {
                let result: Option<HashSet<char>> = shifts
                    .iter()
                    .map(|&shift| {
                        let (x, y) = shift_point(start, shift);
                        grid.get(x, y).copied()
                    })
                    .collect();
                result.is_some_and(|s| s == required_neighbors)
            };
            is_valid(&[(1, 1), (-1, -1)]) && is_valid(&[(1, -1), (-1, 1)])
        })
        .count();
    println!("ans2 = {ans2}");

    Ok(())
}
