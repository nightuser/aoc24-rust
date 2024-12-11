use std::cmp::Ordering;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut values: Vec<i32> = Vec::new();
    let mut indegrees: Vec<i32> = Vec::new();
    let mut neighbors: Vec<Vec<usize>> = Vec::new();
    let mut sources: Vec<usize> = Vec::new();

    let mut index = 0;
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let width = line.len();
        for (x, c) in line.char_indices() {
            let value = c.to_digit(10).unwrap() as i32;
            if value == 0 {
                sources.push(index);
            }
            values.push(value);
            indegrees.push(0);
            neighbors.push(Vec::with_capacity(4));
            let mut other_indices: Vec<usize> = Vec::with_capacity(2);
            if x > 0 {
                other_indices.push(index - 1);
            }
            if y > 0 {
                other_indices.push(index - width);
            }
            for other_index in other_indices {
                match value.cmp(&values[other_index]) {
                    Ordering::Less => {
                        neighbors[index].push(other_index);
                        indegrees[other_index] += 1;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        neighbors[other_index].push(index);
                        indegrees[index] += 1;
                    }
                }
            }
            index += 1;
        }
    }

    let mut queue: VecDeque<usize> = VecDeque::from_iter(
        indegrees
            .iter()
            .enumerate()
            .filter_map(|(index, &indegree)| (indegree == 0).then_some(index)),
    );
    let mut sorted_vertices: Vec<usize> = Vec::with_capacity(values.len());
    while let Some(index) = queue.pop_front() {
        sorted_vertices.push(index);
        for &neighbor in neighbors[index].iter() {
            indegrees[neighbor] -= 1;
            if indegrees[neighbor] == 0 {
                queue.push_back(neighbor);
            }
        }
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut dist = vec![0; values.len()];
    let mut num_paths = vec![0; values.len()];
    for start in sources {
        dist.fill(-1);
        dist[start] = 0;
        num_paths.fill(0);
        num_paths[start] = 1;
        let mut best = 0;
        let mut best_count = 0;
        let mut trails_count = 0;
        for &index in sorted_vertices.iter() {
            if dist[index] == -1 {
                continue;
            }
            if values[index] == 9 {
                match dist[index].cmp(&best) {
                    Ordering::Less => {}
                    Ordering::Equal => {
                        best_count += 1;
                        trails_count += num_paths[index];
                    }
                    Ordering::Greater => {
                        best = dist[index];
                        best_count = 1;
                        trails_count = num_paths[index];
                    }
                }
            }
            for &neighbor in neighbors[index].iter() {
                let new_dist = dist[index] + 1;
                match dist[neighbor].cmp(&new_dist) {
                    Ordering::Less => {
                        dist[neighbor] = new_dist;
                        num_paths[neighbor] = num_paths[index];
                    }
                    Ordering::Equal => num_paths[neighbor] += num_paths[index],
                    Ordering::Greater => {}
                }
            }
        }
        ans1 += best_count;
        ans2 += trails_count;
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
}
