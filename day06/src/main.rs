use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

type Point = (usize, usize);
type Position = (Point, Dir);
type Obstructions = Vec<Vec<usize>>;

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Unknown,
    NotLoop,
    Loop,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    pub fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    pub fn to_index(self, point_index: usize) -> usize {
        4 * point_index + self as usize
    }
}

fn lower_bound<T: Ord + Clone>(haystack: &[T], needle: T, temp_bound: Option<T>) -> Option<T> {
    let index = haystack
        .binary_search_by(|x| match x.cmp(&needle) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    let result = haystack.get(index.wrapping_sub(1)).cloned();
    temp_bound
        .and_then(|t| (t < needle && result.clone().is_none_or(|result| t > result)).then_some(t))
        .or(result)
}

fn upper_bound<T: Ord + Clone>(haystack: &[T], needle: T, temp_bound: Option<T>) -> Option<T> {
    let index = haystack
        .binary_search_by(|x| match x.cmp(&needle) {
            Ordering::Equal => Ordering::Less,
            ord => ord,
        })
        .unwrap_err();
    let result = haystack.get(index).cloned();
    temp_bound
        .and_then(|t| (t > needle && result.clone().is_none_or(|result| t < result)).then_some(t))
        .or(result)
}

enum GridState {
    OutOfBounds,
    Obstruction,
    Point(Point),
}

struct TemporaryObstruction<'a> {
    grid: &'a Grid,
    temp_obstruction: Point,
}

impl TemporaryObstruction<'_> {
    pub fn stops(&self, init_pos: Position) -> impl Iterator<Item = Position> + use<'_> {
        let (t_x, t_y) = self.temp_obstruction;
        iter::successors(Some(init_pos), move |&((x, y), dir)| {
            let next = match dir {
                Dir::Up => lower_bound(
                    &self.grid.vert_obstructions[x],
                    y,
                    (x == t_x).then_some(t_y),
                )
                .map(|o_y| (x, o_y + 1)),
                Dir::Right => {
                    upper_bound(&self.grid.hor_obstructions[y], x, (y == t_y).then_some(t_x))
                        .map(|o_x| (o_x - 1, y))
                }
                Dir::Down => upper_bound(
                    &self.grid.vert_obstructions[x],
                    y,
                    (x == t_x).then_some(t_y),
                )
                .map(|o_y| (x, o_y - 1)),
                Dir::Left => {
                    lower_bound(&self.grid.hor_obstructions[y], x, (y == t_y).then_some(t_x))
                        .map(|o_x| (o_x + 1, y))
                }
            };
            next.map(|next| (next, dir.turn_right()))
        })
        .skip(1) // Skip initial position.
    }
}

struct Grid {
    width: usize,
    height: usize,
    vert_obstructions: Obstructions,
    hor_obstructions: Obstructions,
}

impl Grid {
    pub fn new<T: IntoIterator<Item = Point>>(
        width: usize,
        height: usize,
        obstructions: T,
    ) -> Self {
        let mut vert_obstructions: Obstructions = vec![vec![]; width];
        let mut hor_obstructions: Obstructions = vec![vec![]; height];
        for (x, y) in obstructions {
            vert_obstructions[x].push(y);
            hor_obstructions[y].push(x);
        }
        vert_obstructions.iter_mut().for_each(|v| v.sort());
        hor_obstructions.iter_mut().for_each(|v| v.sort());
        Grid {
            width,
            height,
            vert_obstructions,
            hor_obstructions,
        }
    }

    pub fn with_obstruction(&self, point: Point) -> TemporaryObstruction<'_> {
        TemporaryObstruction {
            grid: self,
            temp_obstruction: point,
        }
    }

    pub fn is_in_bounds(&self, (x, y): Point) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    pub fn is_obstruction(&self, (x, y): Point) -> bool {
        self.vert_obstructions[x].binary_search(&y).is_ok()
    }

    pub fn step(&self, (current, dir): Position) -> GridState {
        let (x, y) = current;
        let next = match dir {
            Dir::Up => (x, y.wrapping_sub(1)),
            Dir::Right => (x.wrapping_add(1), y),
            Dir::Down => (x, y.wrapping_add(1)),
            Dir::Left => (x.wrapping_sub(1), y),
        };
        if !self.is_in_bounds(next) {
            GridState::OutOfBounds
        } else if self.is_obstruction(next) {
            GridState::Obstruction
        } else {
            GridState::Point(next)
        }
    }

    pub fn point_index(&self, (x, y): Point) -> usize {
        y * self.width + x
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
    let (width, height) = (x, y);
    let start = start.unwrap();
    let grid = Grid::new(width, height, obstructions);

    let mut path_points = vec![false; width * height];
    let mut is_loop = vec![Status::Unknown; width * height];
    let mut visited = vec![false; width * height * 4];
    let mut tmp_visited = vec![false; width * height * 4];
    let mut current = start;
    let mut dir = Dir::Up;
    loop {
        let current_index = grid.point_index(current);
        path_points[current_index] = true;
        let pos = (current, dir);
        let pos_index = dir.to_index(current_index);
        visited[pos_index] = true;

        let next = match grid.step(pos) {
            GridState::OutOfBounds => {
                break;
            }
            GridState::Obstruction => {
                dir = dir.turn_right();
                continue;
            }
            GridState::Point(next) => next,
        };
        current = next;
        let next_index = next.1 * width + next.0;
        if next == start || is_loop[next_index] != Status::Unknown {
            continue;
        }
        let extended_grid = grid.with_obstruction(next);
        let mut found = Status::NotLoop;
        for (new_point, new_dir) in extended_grid.stops(pos) {
            let new_pos_index = new_dir.to_index(grid.point_index(new_point));
            if visited[new_pos_index] || tmp_visited[new_pos_index] {
                found = Status::Loop;
                break;
            }
            tmp_visited[new_pos_index] = true;
        }
        tmp_visited.fill(false);
        is_loop[next_index] = found;
    }

    let ans1 = path_points.into_iter().filter(|b| *b).count();
    let ans2 = is_loop.into_iter().filter(|b| *b == Status::Loop).count();
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
