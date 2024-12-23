use std::collections::VecDeque;
use std::env;
use std::ffi::{c_char, c_double, c_float, c_int};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

// See https://github.com/blas-lapack-rs/accelerate-src
extern "C" {
    fn ssymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *const c_float,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );

    // The return type is `double` because of the bug in Apple's Accelerate.
    // See https://stackoverflow.com/a/77017238
    fn sdot_(
        n: *const c_int,
        x: *const c_float,
        incx: *const c_int,
        y: *const c_float,
        incy: *const c_int,
    ) -> c_double;
}

const ALPHABET_SIZE: usize = 26;
const TABLE_DIM: usize = ALPHABET_SIZE * ALPHABET_SIZE;
const TABLE_SIZE: usize = TABLE_DIM * TABLE_DIM;

type GraphMatrix = Box<[c_float]>;
type GraphList = HashMap<usize, HashSet<usize>>;

fn to_key(s: &[u8]) -> usize {
    debug_assert_eq!(s.len(), 2);
    (usize::from(s[0] - b'a') * 26) + usize::from(s[1] - b'a')
}

fn from_key(k: usize) -> String {
    debug_assert!(k < TABLE_DIM);
    let (a, b) = (k / 26, k % 26);
    str::from_utf8(&[a as u8 + b'a', b as u8 + b'a'])
        .unwrap()
        .to_string()
}

fn triangles(table: &GraphMatrix, square_tmp: &mut GraphMatrix) -> i32 {
    unsafe {
        ssymm_(
            &(b'L' as c_char),
            &(b'U' as c_char),
            &(TABLE_DIM as c_int),
            &(TABLE_DIM as c_int),
            &1.0,
            table.as_ptr(),
            &(TABLE_DIM as c_int),
            table.as_ptr(),
            &(TABLE_DIM as c_int),
            &0.0,
            square_tmp.as_mut_ptr(),
            &(TABLE_DIM as c_int),
        );
        let sum = sdot_(
            &(TABLE_SIZE as c_int),
            square_tmp.as_ptr(),
            &1,
            table.as_ptr(),
            &1,
        ) as i32;
        debug_assert_eq!(sum % 6, 0);
        sum / 6
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut table: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let mut table_no_t: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let mut graph: GraphList = GraphList::new();
    let t_range = to_key(b"ta")..=to_key(b"tz");
    for line in reader.lines() {
        let line = line.unwrap();
        let (i, j) = line
            .into_bytes()
            .splitn(2, |c| *c == b'-')
            .map(to_key)
            .collect_tuple()
            .unwrap();
        table[i * TABLE_DIM + j] = 1.0;
        table[j * TABLE_DIM + i] = 1.0;
        if !t_range.contains(&i) && !t_range.contains(&j) {
            table_no_t[i * TABLE_DIM + j] = 1.0;
            table_no_t[j * TABLE_DIM + i] = 1.0;
        }
        graph.entry(i).or_default().insert(j);
        graph.entry(j).or_default().insert(i);
    }
    let mut square_tmp: GraphMatrix = vec![0.0; TABLE_SIZE].into_boxed_slice();
    let trace = triangles(&table, &mut square_tmp);
    let trace_no_t = triangles(&table_no_t, &mut square_tmp);
    let ans1 = trace - trace_no_t;
    println!("ans1 = {ans1}");

    let mut queue: VecDeque<(GraphList, Vec<usize>)> = VecDeque::new();
    queue.push_back((graph, Vec::new()));
    let mut best: Vec<usize> = Vec::new();
    while let Some((mut graph, clique)) = queue.pop_front() {
        if graph.is_empty() {
            if clique.len() > best.len() {
                best = clique;
            }
            continue;
        }
        let candidate = *graph.iter().min_by_key(|(_, ns)| ns.len()).unwrap().0;
        let neighbors = &graph[&candidate];
        if clique.len() + 1 + neighbors.len() > best.len() {
            let mut graph_on_neighbors = GraphList::new();
            for &i in neighbors.iter() {
                graph_on_neighbors.insert(i, graph[&i].intersection(neighbors).cloned().collect());
            }
            queue.push_back((
                graph_on_neighbors,
                clique.iter().cloned().chain(Some(candidate)).collect(),
            ));
        }
        if clique.len() + graph.len() - 1 > best.len() {
            for j in neighbors.clone() {
                graph.get_mut(&j).unwrap().remove(&candidate);
            }
            graph.remove(&candidate);
            queue.push_back((graph, clique.clone()));
        }
    }
    best.sort();
    let ans2 = best.into_iter().map(from_key).join(",");
    println!("ans2 = {ans2}");
}
