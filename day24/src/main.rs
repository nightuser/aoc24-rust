use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Entry = (String, Node, String, String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    And,
    Or,
    Xor,
}

// See diagram.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Func {
    C(i32),
    D(i32),
    E(i32),
    S(i32),
    T(i32),
}

struct NodeFunc {
    node_to_func: HashMap<String, Func>,
    func_to_node: HashMap<Func, String>,
}

impl NodeFunc {
    pub fn new() -> Self {
        Self {
            node_to_func: HashMap::new(),
            func_to_node: HashMap::new(),
        }
    }

    pub fn add(&mut self, node: &str, func: Func) {
        self.node_to_func.insert(node.to_string(), func);
        self.func_to_node.insert(func, node.to_string());
    }

    pub fn swap(&mut self, act: &str, exp: &str) {
        let old_act = self.node_to_func.get(act).cloned();
        let old_exp = self.node_to_func.get(exp).cloned();
        if let Some(old_act) = old_act {
            self.add(exp, old_act);
        }
        if let Some(old_exp) = old_exp {
            self.add(act, old_exp);
        }
    }

    pub fn contains(&self, node: &str) -> bool {
        self.node_to_func.contains_key(node)
    }

    pub fn get(&self, node: &str) -> Func {
        self.node_to_func[node]
    }

    pub fn get_reverse(&self, func: Func) -> String {
        self.func_to_node[&func].clone()
    }
}

fn part1(entries: &[Entry], mut values: HashMap<String, u8>, mut starts: Vec<String>) -> u64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut indegrees: HashMap<String, u8> = HashMap::new();
    let mut node_types: HashMap<String, Node> = HashMap::new();
    let mut deps: HashMap<String, (String, String)> = HashMap::new();
    for (lhs, node_type, rhs, target) in entries {
        graph
            .entry(lhs.to_string())
            .or_default()
            .push(target.to_string());
        graph
            .entry(rhs.to_string())
            .or_default()
            .push(target.to_string());
        indegrees.insert(target.to_string(), 2);
        node_types.insert(target.to_string(), *node_type);
        deps.insert(target.to_string(), (lhs.to_string(), rhs.to_string()));
    }

    let mut topo_order: Vec<String> = Vec::new();
    while let Some(node) = starts.pop() {
        topo_order.push(node.clone());
        if graph.contains_key(&node) {
            for next in graph[&node].iter() {
                indegrees.entry(next.clone()).and_modify(|d| *d -= 1);
                if indegrees[next] == 0 {
                    starts.push(next.clone());
                }
            }
        }
    }

    for node in topo_order.iter() {
        if !deps.contains_key(node) {
            continue;
        }
        let (lhs, rhs) = &deps[node];
        let lhs_value = values[lhs];
        let rhs_value = values[rhs];
        let new_value = match node_types[node] {
            Node::And => lhs_value & rhs_value,
            Node::Or => lhs_value | rhs_value,
            Node::Xor => lhs_value ^ rhs_value,
        };
        values.insert(node.clone(), new_value);
    }

    let mut ans1 = 0;
    for i in (0..64).rev() {
        ans1 = (ans1 << 1) | u64::from(*values.get(&format!("z{i:02}")).unwrap_or(&0));
    }
    ans1
}

fn apply_swap(entries: &mut [Entry], node_func: &mut NodeFunc, act: &str, exp: &str) {
    for (_, _, _, target) in entries.iter_mut() {
        if target == act {
            *target = exp.to_string();
        } else if target == exp {
            *target = act.to_string();
        }
    }
    node_func.swap(act, exp);
}

fn get_next(entries: &[Entry], node_func: &NodeFunc) -> Vec<Entry> {
    let mut active: Vec<Entry> = Vec::new();
    for (lhs, cmd, rhs, target) in entries.iter() {
        if node_func.contains(target) {
            continue;
        }
        if !node_func.contains(lhs) || !node_func.contains(rhs) {
            continue;
        }
        if node_func.get(lhs) > node_func.get(rhs) {
            active.push((rhs.clone(), *cmd, lhs.clone(), target.clone()));
        } else {
            active.push((lhs.clone(), *cmd, rhs.clone(), target.clone()));
        }
    }
    active.sort();
    active
}

// The input is a standard binary addition circuit.
// See e.g. [https://www.101computing.net/binary-additions-using-logic-gates/].
// Given X and Y:
// * C = X & Y -- temp. carry;
// * S = X ^ Y -- temp. sum;
// * D = Cprev & S -- another temp. carry;
// * T = Cprev ^ S  -- sum;
// * E = C | D -- carry.
fn part2(entries: &mut [Entry]) -> String {
    let mut node_func = NodeFunc::new();

    for (lhs, cmd, rhs, target) in entries.iter() {
        if lhs.chars().nth(1).unwrap().is_ascii_digit() && lhs[1..] == rhs[1..] {
            let id = lhs[1..].parse().unwrap();
            match cmd {
                Node::And => {
                    node_func.add(target, Func::C(id));
                }
                Node::Xor => {
                    node_func.add(target, Func::S(id));
                }
                Node::Or => panic!(),
            }
        }
    }
    let mut swaps: Vec<String> = Vec::new();
    let mut step = 1;
    loop {
        let mut data = get_next(entries, &node_func);
        if data.is_empty() {
            break;
        }
        assert_eq!(data.len(), 2);
        let (lhs_other, cmd2, rhs_other, mut target2) = data.pop().unwrap();
        let (lhs, cmd1, rhs, mut target1) = data.pop().unwrap();
        assert_eq!(cmd1, Node::And);
        assert_eq!(cmd2, Node::Xor);
        assert_eq!(lhs, lhs_other);
        assert_eq!(rhs, rhs_other);

        let exp_target = format!("z{step:02}");
        if target2 != exp_target {
            apply_swap(entries, &mut node_func, &target2, &exp_target);
            swaps.push(target2.clone());
            swaps.push(exp_target.clone());
            if target1 == exp_target {
                target1 = target2;
            }
            target2 = exp_target;
        }

        let exps: HashSet<Func> = HashSet::from([
            if step > 1 {
                Func::E(step - 1)
            } else {
                Func::C(0)
            },
            Func::S(step),
        ]);
        let acts = HashSet::from([node_func.get(&lhs), node_func.get(&rhs)]);
        if exps != acts {
            let intersection: Vec<_> = exps.intersection(&acts).collect();
            assert_eq!(intersection.len(), 1); // Otherwise becomes much harder.
            for (act, exp) in acts.difference(&exps).zip(exps.difference(&acts)) {
                let real_act = node_func.get_reverse(*act);
                let real_exp = node_func.get_reverse(*exp);
                apply_swap(entries, &mut node_func, &real_act, &real_exp);
                swaps.push(real_act);
                swaps.push(real_exp);
            }
        }

        node_func.add(&target1, Func::D(step));
        node_func.add(&target2, Func::T(step));

        let mut data = get_next(entries, &node_func);
        assert_eq!(data.len(), 1);
        let (lhs, cmd, rhs, target) = data.pop().unwrap();
        assert_eq!(cmd, Node::Or);

        let exps: HashSet<Func> = HashSet::from([Func::C(step), Func::D(step)]);
        let acts = HashSet::from([node_func.get(&lhs), node_func.get(&rhs)]);
        assert_eq!(exps, acts); // Should perform the swap above if doesn't hold.

        node_func.add(&target, Func::E(step));

        step += 1;
    }
    assert_eq!(swaps.len(), 8);
    swaps.sort();
    swaps.join(",")
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let mut reader = BufReader::new(File::open(input).unwrap());

    let mut values: HashMap<String, u8> = HashMap::new();
    let mut starts: Vec<String> = Vec::new();
    let mut entries: Vec<Entry> = Vec::new();

    for line in (&mut reader).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let (var, value) = line.split(": ").collect_tuple().unwrap();
        let value = value.parse().unwrap();
        starts.push(var.to_string());
        values.insert(var.to_string(), value);
    }
    for line in reader.lines() {
        let line = line.unwrap();
        let (lhs, cmd, rhs, _, target) = line.split_whitespace().collect_tuple().unwrap();
        let node_type = match cmd {
            "AND" => Node::And,
            "OR" => Node::Or,
            "XOR" => Node::Xor,
            _ => panic!(),
        };
        entries.push((
            lhs.to_string(),
            node_type,
            rhs.to_string(),
            target.to_string(),
        ));
    }
    let ans1 = part1(&entries, values, starts);
    println!("ans1 = {ans1}");
    let ans2 = part2(&mut entries);
    println!("ans2 = {ans2}");
}
