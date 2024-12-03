use std::env;
use std::fs;

use anyhow::anyhow;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let input = env::args_os()
        .nth(1)
        .ok_or(anyhow!("valid input argument"))?;
    let contents = fs::read_to_string(input)?;
    let cmd_re =
        Regex::new(r"(?<cmd>mul|do|don't)\((?:(?<lhs>\d+),(?<rhs>\d+))?\)").expect("valid regex");
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut active = true;
    for c in cmd_re.captures_iter(&contents) {
        match &c["cmd"] {
            "mul" => {
                let lhs = c["lhs"].parse::<i32>()?;
                let rhs = c["rhs"].parse::<i32>()?;
                let output = lhs * rhs;
                ans1 += output;
                if active {
                    ans2 += output;
                }
            }
            "do" => active = true,
            "don't" => active = false,
            _ => unreachable!(),
        }
    }
    println!("ans1 = {ans1}");
    println!("ans1 = {ans2}");
    Ok(())
}
