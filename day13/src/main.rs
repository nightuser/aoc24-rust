use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

const SHIFT: i64 = 10000000000000;

fn parse_line(re: &Regex, line: String) -> (i64, i64) {
    let caps = re.captures(&line).unwrap();
    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let button_re = Regex::new(r"^Button (?:A|B): X\+(\d+), Y\+(\d+)$").unwrap();
    let target_re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
    let mut lines = reader.lines();
    let mut ans1 = 0;
    let mut ans2 = 0;
    loop {
        let (a_x, a_y) = parse_line(&button_re, lines.next().unwrap().unwrap());
        let (b_x, b_y) = parse_line(&button_re, lines.next().unwrap().unwrap());
        let (orig_t_x, orig_t_y) = parse_line(&target_re, lines.next().unwrap().unwrap());

        let det = a_x * b_y - b_x * a_y;
        if det == 0 {
            // Requires some proper minimization for cases like:
            //
            // ```text
            // Button A: X+2, Y+0
            // Button B: X+3, Y+0
            // Prize: X=11, Y=0
            //
            // Button A: X+3, Y+0
            // Button B: X+2, Y+0
            // Prize: X=11, Y=0
            // ```
            panic!("Not present in the input");
        }
        for (t_x, t_y, ans) in [
            (orig_t_x, orig_t_y, &mut ans1),
            (SHIFT + orig_t_x, SHIFT + orig_t_y, &mut ans2),
        ] {
            let det1 = t_x * b_y - b_x * t_y;
            let det2 = a_x * t_y - t_x * a_y;
            if det1 % det != 0 || det2 % det != 0 {
                continue;
            }
            let a_presses = det1 / det;
            let b_presses = det2 / det;
            if a_presses < 0 || b_presses < 0 {
                continue;
            }
            *ans += 3 * a_presses + b_presses;
        }

        if lines.next().is_none() {
            break;
        }
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
}
