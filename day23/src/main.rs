use std::collections::VecDeque;
use std::env;
use std::ffi::{c_char, c_double, c_int};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

extern "C" {
    fn dsymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        b: *const c_double,
        ldb: *const c_int,
        beta: *const c_double,
        c: *mut c_double,
        ldc: *const c_int,
    );
    fn ddot_(
        n: *const c_int,
        x: *const c_double,
        incx: *const c_int,
        y: *const c_double,
        incy: *const c_int,
    ) -> c_double;
}

const ALPHABET_SIZE: usize = 26;
const TABLE_DIM: usize = ALPHABET_SIZE * ALPHABET_SIZE;
const TABLE_SIZE: usize = TABLE_DIM * TABLE_DIM;

type GraphMatrix = Vec<c_double>;
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

// See https://github.com/blas-lapack-rs/accelerate-src
// Also see https://forums.developer.apple.com/forums/thread/717757 :
// `sdot_` always return 0 when using Accelerate?!
fn mult(table: &GraphMatrix, square_tmp: &mut GraphMatrix) -> i32 {
    unsafe {
        dsymm_(
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
        let sum = ddot_(
            &(TABLE_SIZE as c_int),
            square_tmp.as_ptr(),
            &1,
            table.as_ptr(),
            &1,
        );
        debug_assert_eq!(sum % 6.0, 0.0);
        (sum as i32) / 6
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut table: GraphMatrix = vec![0.0; TABLE_SIZE];
    let mut table_no_t: GraphMatrix = vec![0.0; TABLE_SIZE];
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
    let mut square_tmp: GraphMatrix = vec![0.0; TABLE_SIZE];
    let trace = mult(&table, &mut square_tmp);
    let trace_no_t = mult(&table_no_t, &mut square_tmp);
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
        let neighbors = graph[&candidate].clone();
        let mut graph_on_neighbors = GraphList::new();
        for &i in neighbors.iter() {
            graph_on_neighbors.insert(i, graph[&i].intersection(&neighbors).cloned().collect());
        }
        for j in neighbors {
            graph.get_mut(&j).unwrap().remove(&candidate);
        }
        graph.remove(&candidate);
        let mut clique_no_antineighbors = clique.clone();
        clique_no_antineighbors.push(candidate);
        queue.push_back((graph, clique));
        queue.push_back((graph_on_neighbors, clique_no_antineighbors));
    }
    best.sort();
    let ans2 = best.into_iter().map(from_key).join(",");
    println!("ans2 = {ans2}");
}
