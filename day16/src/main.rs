use std::cmp::Ordering::{Equal, Greater};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use arrayvec::ArrayVec;

type Predecessors = ArrayVec<usize, 3>;
type Neighbors = ArrayVec<(i32, usize), 3>;

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;
    let mut neighbors: Vec<Neighbors> = Vec::new();
    let mut prev_walls: Vec<bool> = Vec::new();
    let mut walls: Vec<bool> = Vec::new();
    let mut index = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let width = line.len();
        if prev_walls.is_empty() {
            prev_walls.resize(width, true);
            walls.reserve_exact(width);
        }
        for (x, (c, is_wall_above)) in line.chars().zip(prev_walls.drain(..)).enumerate() {
            let is_wall = c == '#';
            walls.push(is_wall);
            if c == 'S' {
                start = Some(index / 4);
            } else if c == 'E' {
                end = Some(index / 4)
            }
            for dir in 0..4 {
                let mut node_neighbors: Neighbors = Neighbors::new();
                if !is_wall {
                    let left_turn = index + ((dir + 3) % 4);
                    let right_turn = index + ((dir + 1) % 4);
                    node_neighbors.push((1000, left_turn));
                    node_neighbors.push((1000, right_turn));
                }
                neighbors.push(node_neighbors);
            }
            if !is_wall {
                let left = index - 4;
                if !walls[x - 1] {
                    neighbors[index].push((1, left));
                    neighbors[left + 2].push((1, index + 2));
                }
                let above = index + 1 - 4 * width;
                if !is_wall_above {
                    neighbors[index + 1].push((1, above));
                    neighbors[above + 2].push((1, index + 3));
                }
            }
            index += 4;
        }
        mem::swap(&mut walls, &mut prev_walls);
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let start_index = 4 * start + 2;
    let mut dist = vec![i32::MAX; neighbors.len()];
    dist[start_index] = 0;
    let mut visited = vec![false; neighbors.len()];
    let mut queue: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    queue.push(Reverse((0, start_index)));
    let mut prev: Vec<Predecessors> = vec![Predecessors::new(); neighbors.len()];
    while let Some(Reverse((_prio, index))) = queue.pop() {
        if visited[index] {
            continue;
        }
        visited[index] = true;
        for &(weight, neighbor) in neighbors[index].iter() {
            let alt = dist[index] + weight;
            match dist[neighbor].cmp(&alt) {
                Greater => {
                    dist[neighbor] = alt;
                    queue.push(Reverse((alt, neighbor)));
                    prev[neighbor].clear();
                    prev[neighbor].push(index);
                }
                Equal => {
                    prev[neighbor].push(index);
                }
                _ => {}
            }
        }
    }

    let mut end_indices: ArrayVec<usize, 4> = ArrayVec::new();
    let mut ans1 = i32::MAX;
    for dir in 0..4 {
        let end_index = 4 * end + dir;
        match ans1.cmp(&dist[end_index]) {
            Greater => {
                ans1 = dist[end_index];
                end_indices.clear();
                end_indices.push(end_index);
            }
            Equal => {
                end_indices.push(end_index);
            }
            _ => {}
        }
    }

    let mut prev_queue: VecDeque<usize> = VecDeque::new();
    visited.fill(false);
    for end_index in end_indices {
        prev_queue.push_back(end_index);
        visited[end_index] = true;
    }
    while let Some(index) = prev_queue.pop_front() {
        for &neighbor in prev[index].iter() {
            if !visited[neighbor] {
                visited[neighbor] = true;
                prev_queue.push_back(neighbor);
            }
        }
    }
    let ans2 = visited.chunks(4).filter(|s| s.iter().any(|b| *b)).count();

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
