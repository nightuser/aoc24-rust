use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

use itertools::{izip, Itertools};
use strum::FromRepr;

type Word = u64;

#[derive(FromRepr)]
#[repr(u8)]
enum Instr {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

#[derive(Debug, Clone)]
enum Expr {
    Literal(Word),
    RegA(Word, Word),
    Mod8(Box<Expr>),
    XorLit(Box<Expr>, Word),
    Div(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Uknown,
}

#[derive(Debug, Clone)]
struct State {
    constraints: Vec<(Expr, bool)>,
    out: Vec<Expr>,
    ip: usize,
    reg_a: Expr,
    reg_b: Expr,
    reg_c: Expr,
}

struct Machine {
    code: Vec<u8>,
}

impl Machine {
    pub fn new(code: Vec<u8>) -> Self {
        Machine { code }
    }

    pub fn run1(&self, mut regs: [Word; 3]) -> Vec<u8> {
        let end = self.code.len();
        let mut ip = 0;
        let mut out: Vec<u8> = Vec::new();
        while ip < end {
            let instr = Instr::from_repr(self.code[ip]).unwrap();
            let literal: Word = self.code[ip + 1].into();
            let combo: Word = if literal & 4 == 0 {
                literal
            } else {
                regs[(literal & 3) as usize]
            };
            match instr {
                Instr::Adv => {
                    regs[0] /= 1 << combo;
                }
                Instr::Bxl => {
                    regs[1] ^= literal;
                }
                Instr::Bst => {
                    regs[1] = combo & 7;
                }
                Instr::Jnz => {
                    if regs[0] != 0 {
                        ip = literal as usize;
                        continue;
                    }
                }
                Instr::Bxc => {
                    regs[1] ^= regs[2];
                }
                Instr::Out => {
                    let result = (combo & 7) as u8;
                    out.push(result);
                }
                Instr::Bdv => {
                    regs[1] = regs[0] / (1 << combo);
                }
                Instr::Cdv => {
                    regs[2] = regs[0] / (1 << combo);
                }
            }
            ip += 2;
        }
        out
    }

    pub fn run2(&self) -> State {
        let end = self.code.len();
        let mut states = vec![State {
            constraints: Vec::new(),
            out: Vec::new(),
            ip: 0,
            reg_a: Expr::RegA(0, Word::BITS.into()),
            reg_b: Expr::Uknown,
            reg_c: Expr::Uknown,
        }];
        let mut new_states: Vec<State> = Vec::new();

        let mut best_state: Option<State> = None;
        while !states.is_empty() {
            for mut state in states.drain(..) {
                if state.ip == end {
                    if state.out.len() == end {
                        if best_state.is_some() {
                            panic!("multiple solutions");
                        }
                        best_state = Some(state);
                    }
                    continue;
                }
                let instr = Instr::from_repr(self.code[state.ip]).unwrap();
                let literal = self.code[state.ip + 1].into();
                let combo = if literal & 4 == 0 {
                    Expr::Literal(literal)
                } else {
                    match literal & 3 {
                        0 => state.reg_a.clone(),
                        1 => state.reg_b.clone(),
                        2 => state.reg_c.clone(),
                        3 => panic!("reserved"),
                        _ => unreachable!(),
                    }
                };
                match instr {
                    Instr::Adv => {
                        state.reg_a = Expr::Div(Box::new(state.reg_a), Box::new(combo));
                    }
                    Instr::Bxl => {
                        state.reg_b = Expr::XorLit(Box::new(state.reg_b), literal);
                    }
                    Instr::Bst => {
                        state.reg_b = Expr::Mod8(Box::new(combo));
                    }
                    Instr::Jnz => {
                        let constraint = state.reg_a.clone();
                        let mut jump_state = state.clone();
                        jump_state.constraints.push((constraint.clone(), true));
                        jump_state.ip = literal as usize;
                        new_states.push(jump_state);
                        state.constraints.push((constraint, false));
                    }
                    Instr::Bxc => {
                        state.reg_b =
                            Expr::Xor(Box::new(state.reg_b), Box::new(state.reg_c.clone()));
                    }
                    Instr::Out => {
                        if state.out.len() == end {
                            // The output size exceeds the target size.
                            continue;
                        }
                        let result = Expr::Mod8(Box::new(combo));
                        state.out.push(result);
                    }
                    Instr::Bdv => {
                        state.reg_b = Expr::Div(Box::new(state.reg_a.clone()), Box::new(combo));
                    }
                    Instr::Cdv => {
                        state.reg_c = Expr::Div(Box::new(state.reg_a.clone()), Box::new(combo));
                    }
                }
                state.ip += 2;
                new_states.push(state);
            }
            mem::swap(&mut states, &mut new_states);
        }
        best_state.unwrap()
    }
}

fn extract_value(line: &str) -> &str {
    line.rsplit_once(' ').unwrap().1
}

fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::Literal(..) | Expr::RegA(..) => expr.clone(),
        Expr::Mod8(inner) => match simplify(inner.as_ref()) {
            Expr::Literal(inner_value) => Expr::Literal(inner_value & 7),
            Expr::RegA(lower, upper) => Expr::RegA(lower, upper.min(lower + 3)),
            inner_simpl => Expr::Mod8(Box::new(inner_simpl)),
        },
        Expr::XorLit(inner, literal) => match simplify(inner.as_ref()) {
            Expr::Literal(inner_value) => Expr::Literal(inner_value ^ Word::from(*literal)),
            inner_simpl => Expr::XorLit(Box::new(inner_simpl), *literal),
        },
        Expr::Div(lhs, rhs) => match (simplify(lhs.as_ref()), simplify(rhs.as_ref())) {
            (Expr::Literal(lhs_value), Expr::Literal(rhs_value)) => {
                Expr::Literal(lhs_value / (1 << rhs_value))
            }
            (Expr::RegA(lower, upper), Expr::Literal(rhs_value)) => {
                Expr::RegA(lower + rhs_value, upper)
            }
            (lhs_simpl, rhs_simpl) => Expr::Div(Box::new(lhs_simpl), Box::new(rhs_simpl)),
        },
        Expr::Xor(lhs, rhs) => match (simplify(lhs.as_ref()), simplify(rhs.as_ref())) {
            (Expr::Literal(lhs_value), Expr::Literal(rhs_value)) => {
                Expr::Literal(lhs_value ^ rhs_value)
            }
            (lhs_simpl, rhs_simpl) => Expr::Xor(Box::new(lhs_simpl), Box::new(rhs_simpl)),
        },
        Expr::Uknown => panic!("malformed input"),
    }
}

fn calculate(expr: &Expr, a_value: Word) -> Word {
    match expr {
        Expr::Literal(value) => *value,
        Expr::RegA(lower, upper) => {
            if (upper - lower) == Word::BITS.into() {
                a_value
            } else {
                let mask = (1 << (upper - lower)) - 1;
                (a_value >> lower) & mask
            }
        }
        Expr::Mod8(inner) => calculate(inner.as_ref(), a_value) & 7,
        Expr::XorLit(inner, literal) => calculate(inner.as_ref(), a_value) ^ *literal as Word,
        Expr::Div(lhs, rhs) => {
            calculate(lhs.as_ref(), a_value) / (1 << calculate(rhs.as_ref(), a_value))
        }
        Expr::Xor(lhs, rhs) => calculate(lhs.as_ref(), a_value) ^ calculate(rhs.as_ref(), a_value),
        Expr::Uknown => panic!("malformed input"),
    }
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut lines = reader.lines();
    let regs: [Word; 3] = (&mut lines)
        .take(3)
        .map(|line| {
            let line = line.unwrap();
            extract_value(&line).parse().unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let line = lines.nth(1).unwrap().unwrap();
    let code: Vec<u8> = extract_value(&line)
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let machine = Machine::new(code.clone());

    let out = machine.run1(regs);
    let ans1 = out.iter().join(",");

    let state = machine.run2();
    let mut equations: Vec<(Expr, Word, Word)> = Vec::with_capacity(state.out.len());
    for (expr, target, offset) in izip!(state.out.iter(), code.iter(), (0..).step_by(3)) {
        let expr_simpl = simplify(expr);
        println!("{expr_simpl:?} should be {target}");
        equations.push((expr_simpl, (*target).into(), offset));
    }

    let mut a_values: Vec<Word> = vec![0];
    let mut new_a_values: Vec<Word> = Vec::new();
    for (expr, target, offset) in equations.into_iter().rev() {
        for a_value in a_values.drain(..) {
            for guess in 0..=7 {
                let a_guess = a_value | (guess << offset);
                let result = calculate(&expr, a_guess);
                if result == target {
                    new_a_values.push(a_guess);
                }
            }
        }
        mem::swap(&mut a_values, &mut new_a_values);
    }
    let ans2 = *a_values.iter().min().unwrap();
    assert_eq!(machine.run1([ans2, 0, 0]), code);

    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
