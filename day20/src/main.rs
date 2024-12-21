use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

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
            width = line.len();
        }
        for c in line.into_bytes() {
            if c == b'S' {
                start = walls.len();
            } else if c == b'E' {
                end = walls.len();
            }
            walls.push(c == b'#');
        }
    }
    let neighbors = |current: usize, shift: usize| {
        let mut potential_neighbors: Vec<usize> = Vec::with_capacity(4);
        let x = current % width;
        if x >= shift {
            potential_neighbors.push(current - shift);
        }
        if x < width - shift {
            potential_neighbors.push(current + shift);
        }
        if current >= shift * width {
            potential_neighbors.push(current - shift * width);
        }
        if current + shift * width < walls.len() {
            potential_neighbors.push(current + shift * width);
        }
        potential_neighbors.into_iter()
    };

    let mut path: Vec<usize> = vec![start];
    let mut current = path[0];
    let mut prev = current;
    while current != end {
        for next in neighbors(current, 1) {
            if walls[next] || prev == next {
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

    let mut ans1 = 0;
    for (i, &index) in path.iter().enumerate() {
        for cheat_next in neighbors(index, 2) {
            if dist[cheat_next] == usize::MAX {
                continue;
            }
            let new_total = i + 2 + dist[cheat_next];
            if new_total < total {
                let saved_time = total - new_total;
                if saved_time >= 100 {
                    ans1 += 1;
                }
            }
        }
    }
    println!("ans1 = {ans1}");

    let mut ans2 = 0;
    let mut current_layer: Vec<usize> = Vec::new();
    let mut next_layer: Vec<usize> = Vec::new();
    let mut visited = vec![false; walls.len()];

    for (i, &index) in path.iter().enumerate() {
        current_layer.clear();
        visited.fill(false);
        current_layer.push(index);
        visited[index] = true;
        for cheat_time in 1..=20 {
            for current_cheat in current_layer.drain(..) {
                for next_cheat in neighbors(current_cheat, 1) {
                    if visited[next_cheat] {
                        continue;
                    }
                    visited[next_cheat] = true;
                    next_layer.push(next_cheat);
                    if dist[next_cheat] != usize::MAX {
                        let new_total = i + cheat_time + dist[next_cheat];
                        if new_total < total {
                            let saved_time = total - new_total;
                            if saved_time >= 100 {
                                ans2 += 1;
                            }
                        }
                    }
                }
            }
            mem::swap(&mut current_layer, &mut next_layer);
        }
    }

    println!("ans2 = {ans2}");
}
