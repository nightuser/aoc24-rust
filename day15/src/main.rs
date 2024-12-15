use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use hashbrown::{HashMap, HashSet};

type Point = (i32, i32);
type Grid = HashMap<Point, u8>;

fn try_moving1(grid: &mut Grid, pos: Point, (dx, dy): Point) -> Point {
    let mut cur = pos;
    let mut participating_boxes: Vec<(Point, u8)> = Vec::new();
    loop {
        cur = (cur.0 + dx, cur.1 + dy);
        if let Some(&c) = grid.get(&cur) {
            if c == b'#' {
                return pos;
            }
        } else {
            break;
        }
        participating_boxes.push((cur, grid[&cur]));
    }
    for (pos, _) in participating_boxes.iter() {
        grid.remove(pos).unwrap();
    }
    for (pos, box_type) in participating_boxes {
        grid.insert((pos.0 + dx, pos.1 + dy), box_type);
    }
    (pos.0 + dx, pos.1 + dy)
}

fn try_moving2(grid: &mut Grid, pos: Point, dy: i32) -> Point {
    let mut front: HashSet<Point> = HashSet::from([pos]);
    let mut participating_boxes: Vec<(Point, u8)> = Vec::new();
    let mut new_front: HashSet<Point> = HashSet::new();
    loop {
        for point in front.drain() {
            let next_point = (point.0, point.1 + dy);
            if let Some(&c) = grid.get(&next_point) {
                if c == b'#' {
                    return pos;
                }
                new_front.insert(next_point);
                let (next_x, next_y) = next_point;
                if c == b'[' {
                    new_front.insert((next_x + 1, next_y));
                } else if c == b']' {
                    new_front.insert((next_x - 1, next_y));
                }
            }
        }
        if new_front.is_empty() {
            break;
        }
        participating_boxes.extend(new_front.iter().map(|point| (*point, grid[point])));
        mem::swap(&mut front, &mut new_front);
    }

    for (pos, _) in participating_boxes.iter() {
        grid.remove(pos).unwrap();
    }
    for (pos, box_type) in participating_boxes {
        grid.insert((pos.0, pos.1 + dy), box_type);
    }
    (pos.0, pos.1 + dy)
}

fn get_gps(grid: &Grid, box_c: u8) -> i32 {
    grid.iter()
        .filter_map(|((x, y), c)| (*c == box_c).then_some(y * 100 + x))
        .sum()
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let mut reader = BufReader::new(File::open(input).unwrap());
    let mut grid1: Grid = HashMap::new();
    let mut grid2: Grid = HashMap::new();
    let mut start1: Option<Point> = None;
    let mut start2: Option<Point> = None;
    for (y, line) in (&mut reader).lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        for (x, c) in line.into_bytes().into_iter().enumerate() {
            let point = (x as i32, y as i32);
            let wide_point1 = ((2 * x) as i32, y as i32);
            let wide_point2 = ((2 * x + 1) as i32, y as i32);
            match c {
                b'#' => {
                    grid1.insert(point, b'#');
                    grid2.insert(wide_point1, b'#');
                    grid2.insert(wide_point2, b'#');
                }
                b'O' => {
                    grid1.insert(point, b'O');
                    grid2.insert(wide_point1, b'[');
                    grid2.insert(wide_point2, b']');
                }
                b'@' => {
                    start1 = Some(point);
                    start2 = Some(wide_point1);
                }
                _ => {}
            }
        }
    }
    let start1 = start1.unwrap();
    let start2 = start2.unwrap();
    let shifts: HashMap<u8, Point> = [
        (b'^', (0, -1)),
        (b'v', (0, 1)),
        (b'<', (-1, 0)),
        (b'>', (1, 0)),
    ]
    .into_iter()
    .collect();
    let mut pos1 = start1;
    let mut pos2 = start2;
    for line in reader.lines() {
        let line = line.unwrap();
        for c in line.into_bytes() {
            let shift = shifts[&c];
            pos1 = try_moving1(&mut grid1, pos1, shift);
            if c == b'<' || c == b'>' {
                pos2 = try_moving1(&mut grid2, pos2, shift);
            } else {
                pos2 = try_moving2(&mut grid2, pos2, shift.1);
            }
        }
    }
    let ans1: i32 = get_gps(&grid1, b'O');
    let ans2: i32 = get_gps(&grid2, b'[');

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
