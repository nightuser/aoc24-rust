use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

type Point = (usize, usize);
type Position = (Point, Dir);
type Obstructions<'a> = Vec<Cow<'a, [usize]>>;

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

fn lower_bound<'a, T: Ord>(haystack: &'a [T], needle: &'a T) -> Option<&'a T> {
    let index = haystack
        .binary_search_by(|x| match x.cmp(needle) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    if index == 0 {
        None
    } else {
        Some(&haystack[index - 1])
    }
}

fn upper_bound<'a, T: Ord>(haystack: &'a [T], needle: &'a T) -> Option<&'a T> {
    let index = haystack
        .binary_search_by(|x| match x.cmp(needle) {
            Ordering::Equal => Ordering::Less,
            ord => ord,
        })
        .unwrap_err();
    if index == haystack.len() {
        None
    } else {
        Some(&haystack[index])
    }
}

struct Path<'a> {
    grid: &'a Grid<'a>,
    current_pos: Option<Position>,
}

impl Iterator for Path<'_> {
    type Item = (Position, Option<Point>);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.current_pos?;
        let ((x, y), dir) = pos;
        let next_point = match dir {
            Dir::Up => (x, y.wrapping_sub(1)),
            Dir::Right => (x.wrapping_add(1), y),
            Dir::Down => (x, y.wrapping_add(1)),
            Dir::Left => (x.wrapping_sub(1), y),
        };
        let next = if !self.grid.is_in_bounds(next_point) {
            self.current_pos = None;
            None
        } else if self.grid.is_obstruction(next_point) {
            self.current_pos = Some(((x, y), dir.turn_right()));
            None
        } else {
            self.current_pos = Some((next_point, dir));
            Some(next_point)
        };
        Some((pos, next))
    }
}

struct Grid<'a> {
    width: usize,
    height: usize,
    vert_obstructions: Obstructions<'a>,
    hor_obstructions: Obstructions<'a>,
}

impl<'a> Grid<'a> {
    pub fn new<T: IntoIterator<Item = Point>>(
        width: usize,
        height: usize,
        obstructions: T,
    ) -> Self {
        let mut vert_obstructions: Obstructions<'a> = vec![Cow::default(); width];
        let mut hor_obstructions: Obstructions<'a> = vec![Cow::default(); height];
        for (x, y) in obstructions {
            vert_obstructions[x].to_mut().push(y);
            hor_obstructions[y].to_mut().push(x);
        }
        vert_obstructions.iter_mut().for_each(|v| v.to_mut().sort());
        hor_obstructions.iter_mut().for_each(|v| v.to_mut().sort());
        Grid {
            width,
            height,
            vert_obstructions,
            hor_obstructions,
        }
    }

    pub fn with_parent(parent: &'a Grid, (x, y): Point) -> Self {
        let mut vert_obstructions = parent.vert_obstructions.clone();
        let mut hor_obstructions = parent.hor_obstructions.clone();
        if let Err(pos) = vert_obstructions[x].binary_search(&y) {
            vert_obstructions[x].to_mut().insert(pos, y);
        }
        if let Err(pos) = hor_obstructions[y].binary_search(&x) {
            hor_obstructions[y].to_mut().insert(pos, x);
        }
        Grid {
            width: parent.width,
            height: parent.height,
            vert_obstructions,
            hor_obstructions,
        }
    }

    pub fn is_in_bounds(&self, (x, y): Point) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    pub fn is_obstruction(&self, (x, y): Point) -> bool {
        self.vert_obstructions[x].binary_search(&y).is_ok()
    }

    pub fn path(&self, init_pos: Position) -> Path<'_> {
        Path {
            grid: self,
            current_pos: Some(init_pos),
        }
    }

    pub fn stops(&self, init_pos: Position) -> impl Iterator<Item = Position> + use<'_> {
        iter::successors(Some(init_pos), |&((x, y), dir)| {
            let next = match dir {
                Dir::Up => {
                    lower_bound(self.vert_obstructions[x].as_ref(), &y).map(|&o_y| (x, o_y + 1))
                }
                Dir::Right => {
                    upper_bound(self.hor_obstructions[y].as_ref(), &x).map(|&o_x| (o_x - 1, y))
                }
                Dir::Down => {
                    upper_bound(self.vert_obstructions[x].as_ref(), &y).map(|&o_y| (x, o_y - 1))
                }
                Dir::Left => {
                    lower_bound(self.hor_obstructions[y].as_ref(), &x).map(|&o_x| (o_x + 1, y))
                }
            };
            next.map(|next| (next, dir.turn_right()))
        })
        .skip(1) // Skip initial position.
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut start = None;
    let mut obstructions: Vec<Point> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        x = 0;
        for c in line.chars() {
            match c {
                '^' => start = Some((x, y)),
                '#' => {
                    obstructions.push((x, y));
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
    for (pos, next) in grid.path((start, Dir::Up)) {
        path_points.insert(pos.0);
        visited.insert(pos);

        let Some(next) = next else {
            continue;
        };
        if next == start || is_loop.contains_key(&next) {
            continue;
        }
        let new_grid = Grid::with_parent(&grid, next);
        let mut found = false;
        for new_pos in new_grid.stops(pos) {
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

#[cfg(test)]
mod tests;
