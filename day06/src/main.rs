use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

type Point = (usize, usize);
type Position = (Point, Dir);
type Obstructions = HashSet<Point>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

struct Grid<'a> {
    width: usize,
    height: usize,
    obstructions: Obstructions,
    parent: Option<&'a Grid<'a>>,
}

impl<'a> Grid<'a> {
    pub fn new<T: Into<Obstructions>>(width: usize, height: usize, obstructions: T) -> Self {
        Grid {
            width,
            height,
            obstructions: obstructions.into(),
            parent: None,
        }
    }

    pub fn with_parent<T: Into<Obstructions>>(parent: &'a Grid, obstructions: T) -> Self {
        Grid {
            width: parent.width,
            height: parent.height,
            obstructions: obstructions.into(),
            parent: Some(parent),
        }
    }

    pub fn is_in_bounds(&self, (x, y): Point) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    pub fn is_obstruction(&self, point: Point) -> bool {
        self.obstructions.contains(&point)
            || self
                .parent
                .is_some_and(|parent| parent.obstructions.contains(&point))
    }

    // Step can be implemented using binary search via jumping to the next obstacle
    pub fn step(&self, ((x, y), dir): Position) -> Point {
        let (dx, dy) = match dir {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        };
        (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))
    }

    pub fn path(&self, init_pos: Position) -> impl Iterator<Item = (Point, Dir)> + use<'_> {
        iter::successors(Some(init_pos), |&pos| {
            let (current, dir) = pos;
            let next = self.step(pos);
            if !self.is_in_bounds(next) {
                None
            } else if self.is_obstruction(next) {
                Some((current, dir.turn_right()))
            } else {
                Some((next, dir))
            }
        })
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut start = None;
    let mut obstructions: HashSet<Point> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        x = 0;
        for c in line.chars() {
            match c {
                '^' => start = Some((x, y)),
                '#' => {
                    obstructions.insert((x, y));
                }
                _ => {}
            }
            x += 1;
        }
        y += 1;
    }
    let start = start.unwrap();
    let grid = Grid::new(x, y, obstructions);

    let mut path_points: HashSet<Point> = HashSet::new();
    let mut is_loop: HashMap<Point, bool> = HashMap::new();
    let mut visited: HashSet<Position> = HashSet::new();
    let mut tmp_visited: HashSet<Position> = HashSet::new();

    for pos in grid.path((start, Dir::Up)) {
        path_points.insert(pos.0);
        visited.insert(pos);

        let next = grid.step(pos);
        if grid.is_obstruction(next) || next == start || is_loop.contains_key(&next) {
            continue;
        }
        let mut found = false;
        let new_grid = Grid::with_parent(&grid, [next]);
        for new_pos in new_grid.path(pos).skip(1) {
            if visited.contains(&new_pos) || tmp_visited.contains(&new_pos) {
                found = true;
                break;
            }
            tmp_visited.insert(new_pos);
        }
        tmp_visited.clear();
        is_loop.insert(next, found);
    }
    let ans1 = path_points.len();
    let ans2 = is_loop.into_values().filter(|b| *b).count();
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
