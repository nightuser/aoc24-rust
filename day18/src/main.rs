use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

type Point = (usize, usize);

const INF: usize = usize::MAX;

struct Grid {
    size: usize,
    grid: Vec<usize>,
    dist: Vec<usize>,
    queue: VecDeque<usize>,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let grid_size = size * size;
        let mut grid = vec![INF; grid_size];
        for x in 0..size {
            grid[x] = 0;
            grid[grid_size - size + x] = 0;
        }
        for y in 0..size {
            grid[y * size] = 0;
            grid[y * size + size - 1] = 0;
        }
        Self {
            size,
            grid,
            dist: vec![0; grid_size],
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, point: Point, value: usize) {
        let index = self.to_index(point);
        if self.grid[index] == INF {
            self.grid[index] = value;
        }
    }

    pub fn run(&mut self, threshold: usize) -> Option<usize> {
        let start_index = self.to_index((1, 1));
        let end_index = self.to_index((self.size - 2, self.size - 2));

        self.dist.fill(INF);
        self.queue.clear();
        self.dist[start_index] = 0;
        self.queue.push_back(start_index);
        while let Some(index) = self.queue.pop_front() {
            if index == end_index {
                return Some(self.dist[end_index]);
            }
            for next_index in [index - 1, index - self.size, index + 1, index + self.size] {
                if self.dist[next_index] != INF || self.grid[next_index] <= threshold {
                    continue;
                }
                self.dist[next_index] = self.dist[index] + 1;
                self.queue.push_back(next_index);
            }
        }
        None
    }

    fn to_index(&self, (x, y): Point) -> usize {
        y * self.size + x
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(&input).unwrap());

    let (size, threshold1) = if input.into_string().is_ok_and(|s| s.starts_with("ex")) {
        (7 + 2, 12)
    } else {
        (71 + 2, 1024)
    };
    let mut grid = Grid::new(size);

    let mut bytes: Vec<Point> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let point: Point = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        bytes.push(point);
        grid.add((point.0 + 1, point.1 + 1), i + 1);
    }

    let ans1 = grid.run(threshold1).unwrap();
    println!("ans1 = {ans1}");

    let mut lower = 0;
    let mut upper = bytes.len() + 1;
    while upper - lower > 1 {
        let threshold = lower + (upper - lower) / 2;
        match grid.run(threshold) {
            Some(_) => {
                lower = threshold;
            }
            None => {
                upper = threshold;
            }
        }
    }
    let ans2 = bytes[lower];
    println!("ans2 = {},{}", ans2.0, ans2.1);
}
