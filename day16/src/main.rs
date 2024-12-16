use std::cmp::Ordering::{Equal, Greater};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use arrayvec::ArrayVec;

type Predecessors = ArrayVec<usize, 3>;
type Neighbors = ArrayVec<(i32, usize), 3>;

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;
    let mut neighbors: Vec<Neighbors> = Vec::new();
    let mut walls: Vec<bool> = Vec::new(); // Only need the last two rows.
    let mut index = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let width = line.len();
        for c in line.chars() {
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
                for (potential_neighbor, dir) in [(index - 4, 0), (index - 4 * width, 1)] {
                    if !walls[(potential_neighbor + dir) / 4] {
                        neighbors[index + dir].push((1, potential_neighbor + dir));
                        neighbors[potential_neighbor + dir + 2].push((1, index + dir + 2));
                    }
                }
            }
            index += 4;
        }
    }
    let start = start.unwrap();
    let end = end.unwrap();

    let start_east = 4 * start + 2;
    let mut dist = vec![i32::MAX; neighbors.len()];
    dist[start_east] = 0;
    let mut visited = vec![false; neighbors.len()];
    let mut queue: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    queue.push(Reverse((0, start_east)));
    let mut prev: Vec<Predecessors> = vec![Predecessors::new(); neighbors.len()];
    let mut end_index: Option<usize> = None;
    while let Some(Reverse((_prio, index))) = queue.pop() {
        if index / 4 == end {
            end_index = Some(index);
            break;
        }
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
    let end_index = end_index.unwrap();
    let ans1 = dist[end_index];

    let mut prev_queue: VecDeque<usize> = VecDeque::new();
    visited.fill(false);
    prev_queue.push_back(end_index);
    visited[end_index] = true;
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
