use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Point = (i32, i32);

struct Grid {
    width: i32,
    height: i32,
    antennas: HashMap<char, Vec<Point>>,
}

impl Grid {
    pub fn new<T: IntoIterator<Item = (char, Point)>>(
        width: i32,
        height: i32,
        antenna_positions: T,
    ) -> Self {
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for (c, point) in antenna_positions {
            antennas.entry(c).or_default().push(point);
        }
        Grid {
            width,
            height,
            antennas,
        }
    }

    pub fn is_in_bounds(&self, (x, y): Point) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    fn line(&self, start: Point, (dx, dy): Point) -> impl Iterator<Item = Point> + use<'_> {
        iter::successors(Some(start), move |&(x, y)| {
            let point = (x + dx, y + dy);
            self.is_in_bounds(point).then_some(point)
        })
    }

    pub fn antinodes(&self) -> (usize, usize) {
        let mut antinodes1: HashSet<Point> = HashSet::new();
        let mut antinodes2: HashSet<Point> = HashSet::new();
        for positions in self.antennas.values() {
            for (&(x1, y1), &(x2, y2)) in positions.iter().tuple_combinations() {
                let (dx, dy) = (x2 - x1, y2 - y1);
                antinodes1.extend(
                    [(x1 - dx, y1 - dy), (x2 + dx, y2 + dy)]
                        .into_iter()
                        .filter(|&p| self.is_in_bounds(p)),
                );
                antinodes2.extend(self.line((x1, y1), (-dx, -dy)));
                antinodes2.extend(self.line((x2, y2), (dx, dy)));
            }
        }
        (antinodes1.len(), antinodes2.len())
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut x = 0;
    let mut y = 0;
    let mut antenna_positions = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        x = 0;
        for c in line.chars() {
            if c.is_alphanumeric() {
                antenna_positions.push((c, (x, y)));
            }
            x += 1;
        }
        y += 1;
    }
    let (width, height) = (x, y);
    let grid = Grid::new(width, height, antenna_positions);
    let (ans1, ans2) = grid.antinodes();
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
