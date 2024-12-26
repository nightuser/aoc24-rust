use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut width = 0;
    let mut walls: Vec<bool> = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if width == 0 {
            width = line.len() + 2;
            walls.extend(iter::repeat_n(true, width));
        }
        walls.push(true);
        for c in line.into_bytes() {
            if c == b'S' {
                start = walls.len();
            } else if c == b'E' {
                end = walls.len();
            }
            walls.push(c == b'#');
        }
        walls.push(true);
    }
    walls.extend(iter::repeat_n(true, width));
    let height = walls.len() / width;

    let mut path: Vec<usize> = vec![start];
    let mut current = path[0];
    let mut prev = current;
    while current != end {
        for next in [current - width, current - 1, current + 1, current + width] {
            if prev == next || walls[next] {
                continue;
            }
            prev = current;
            current = next;
            break;
        }
        path.push(current);
    }
    let total = path.len();
    let mut dist: Vec<usize> = vec![usize::MAX; walls.len()];
    for (i, &index) in path.iter().enumerate() {
        dist[index] = total - i;
    }

    let (ans1, ans2) = path
        .par_iter()
        .enumerate()
        .map(|(i, &path_index)| {
            let (x, y) = (path_index % width, path_index / width);
            let mut res1 = 0;
            let mut res2 = 0;
            for cheat_y in y.saturating_sub(20)..=(y + 20).min(height - 1) {
                let dy = y.abs_diff(cheat_y);
                let leftover = 20 - dy;
                for cheat_x in x.saturating_sub(leftover)..=(x + leftover).min(width - 1) {
                    let cheat_index = cheat_y * width + cheat_x;
                    if walls[cheat_index] {
                        continue;
                    }
                    let dx = x.abs_diff(cheat_x);
                    let cheat_time = dx + dy;
                    let new_total = i + cheat_time + dist[cheat_index];
                    if new_total < total && total - new_total >= 100 {
                        if cheat_time <= 2 {
                            res1 += 1;
                        }
                        res2 += 1;
                    }
                }
            }
            (res1, res2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
