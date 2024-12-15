use core::str;
use std::cmp::Ordering::{Greater, Less};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

type Point = (i32, i32);
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let line_re = Regex::new(r"^p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)$").unwrap();
    let mut robots: Vec<(Point, Point)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let caps = line_re.captures(&line).unwrap();
        let (p_x, p_y): Point = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let (v_x, v_y): Point = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
        robots.push(((p_x, p_y), (v_x, v_y)));
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for (p, v) in robots.iter() {
        let (x, y) = (
            (p.0 + 100 * v.0).rem_euclid(WIDTH),
            (p.1 + 100 * v.1).rem_euclid(HEIGHT),
        );
        match (x.cmp(&(&WIDTH / 2)), y.cmp(&(&HEIGHT / 2))) {
            (Less, Less) => q1 += 1,
            (Less, Greater) => q2 += 1,
            (Greater, Less) => q3 += 1,
            (Greater, Greater) => q4 += 1,
            _ => {}
        }
    }
    let ans1 = q1 * q2 * q3 * q4;

    let mut ans2 = 0;
    let mut grid: Vec<u8> = vec![0; (WIDTH * HEIGHT) as usize];
    for seconds in 0.. {
        grid.fill(b' ');
        let mut good = true;
        for (p, v) in robots.iter() {
            let (x, y) = (
                (p.0 + seconds * v.0).rem_euclid(WIDTH),
                (p.1 + seconds * v.1).rem_euclid(HEIGHT),
            );
            let index = (y * WIDTH + x) as usize;
            if grid[index] != b' ' {
                good = false;
                break;
            }
            grid[index] = b'O';
        }
        if good {
            for index in (0..grid.len()).step_by(WIDTH as usize) {
                let row = &grid[index..index + WIDTH as usize];
                println!("{}", str::from_utf8(row).unwrap());
            }
            ans2 = seconds;
            break;
        }
    }
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
