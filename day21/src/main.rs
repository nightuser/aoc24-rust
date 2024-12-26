use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::sync::{LazyLock, Mutex};

use hashbrown::HashMap;

type Counter = HashMap<String, usize>;
type Shift = (isize, isize);
type Shifts = Vec<Shift>;

const PAD_WIDTH: usize = 3;
const STEPS: i64 = 25;

static ROBOPAD: &[u8; 6] = b"X^A<v>";
static KEYPAD: &[u8; 12] = b"789456123X0A";
static ROBOPAD_INDICES: LazyLock<HashMap<u8, usize>> =
    LazyLock::new(|| HashMap::from_iter(ROBOPAD.iter().copied().zip(0..)));
static KEYPAD_INDICES: LazyLock<HashMap<u8, usize>> =
    LazyLock::new(|| HashMap::from_iter(KEYPAD.iter().copied().zip(0..)));

fn to_coord(index: usize) -> (usize, usize) {
    (index % PAD_WIDTH, index / PAD_WIDTH)
}

fn parse_code(code: &str) -> Shifts {
    let mut shifts: Shifts = Shifts::new();
    let (mut prev_x, mut prev_y) = to_coord(KEYPAD_INDICES[&b'A']);
    for c in code.bytes() {
        let (x, y) = to_coord(KEYPAD_INDICES[&c]);
        shifts.push((
            (x as isize) - (prev_x as isize),
            (y as isize) - (prev_y as isize),
        ));
        (prev_x, prev_y) = (x, y);
    }
    shifts
}

fn extract_shifts(inp: &str) -> Shifts {
    let mut shifts: Shifts = Shifts::new();
    let (mut dx, mut dy) = (0, 0);
    for c in inp.bytes() {
        match c {
            b'^' => dy -= 1,
            b'v' => dy += 1,
            b'<' => dx -= 1,
            b'>' => dx += 1,
            b'A' => {
                shifts.push((dx, dy));
                (dx, dy) = (0, 0);
            }
            _ => panic!("unknown shift"),
        }
    }
    shifts
}

fn process_command(is_keypad: bool, cur: usize, (dx, dy): Shift) -> (String, usize) {
    use std::cmp::Ordering::*;
    type CommandsCacheState = (bool, usize, Shift);
    static COMMANDS_CACHE: LazyLock<Mutex<HashMap<CommandsCacheState, String>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    let nxt = cur.wrapping_add_signed(dy * (PAD_WIDTH as isize) + dx);
    let cmd = COMMANDS_CACHE
        .lock()
        .unwrap()
        .entry((is_keypad, cur, (dx, dy)))
        .or_insert_with(|| match (dx.cmp(&0), dy.cmp(&0)) {
            (Less, Less) => {
                if is_keypad
                    && ((KEYPAD[cur] == b'A' && dx == -2) || (KEYPAD[cur] == b'0' && dx == -1))
                {
                    format!(
                        "<{}v<{}>>^A",
                        "A".repeat(dy.unsigned_abs()),
                        "A".repeat(dx.unsigned_abs())
                    )
                } else {
                    format!(
                        "v<<{}>^{}>A",
                        "A".repeat(dx.unsigned_abs()),
                        "A".repeat(dy.unsigned_abs())
                    )
                }
            }
            (Less, Equal) => {
                format!("<v<{}>>^A", "A".repeat(dx.unsigned_abs()))
            }
            (Less, Greater) => {
                if !is_keypad
                    && ((ROBOPAD[cur] == b'A' && dx == -2) || (ROBOPAD[cur] == b'^' && dx == -1))
                {
                    format!(
                        "<v{}<{}>>^A",
                        "A".repeat(dy.unsigned_abs()),
                        "A".repeat(dx.unsigned_abs())
                    )
                } else {
                    format!(
                        "v<<{}>{}^>A",
                        "A".repeat(dx.unsigned_abs()),
                        "A".repeat(dy.unsigned_abs())
                    )
                }
            }
            (Equal, Less) => {
                format!("<{}>A", "A".repeat(dy.unsigned_abs()))
            }
            (Equal, Equal) => "A".to_string(),
            (Equal, Greater) => {
                format!("v<{}^>A", "A".repeat(dy.unsigned_abs()))
            }
            (Greater, Less) => {
                if !is_keypad
                    && ((ROBOPAD[nxt] == b'A' && dx == 2) || (ROBOPAD[nxt] == b'^' && dx == 1))
                {
                    format!(
                        "v{}<^{}>A",
                        "A".repeat(dx.unsigned_abs()),
                        "A".repeat(dy.unsigned_abs())
                    )
                } else {
                    format!(
                        "<{}>v{}^A",
                        "A".repeat(dy.unsigned_abs()),
                        "A".repeat(dx.unsigned_abs())
                    )
                }
            }
            (Greater, Equal) => {
                format!("v{}^A", "A".repeat(dx.unsigned_abs()))
            }
            (Greater, Greater) => {
                if is_keypad
                    && ((KEYPAD[nxt] == b'A' && dx == 2) || (KEYPAD[nxt] == b'0' && dx == 1))
                {
                    format!(
                        "v{}<{}^>A",
                        "A".repeat(dx.unsigned_abs()),
                        "A".repeat(dy.unsigned_abs())
                    )
                } else {
                    format!(
                        "v<{}>{}^A",
                        "A".repeat(dy.unsigned_abs()),
                        "A".repeat(dx.unsigned_abs())
                    )
                }
            }
        })
        .clone();
    (cmd, nxt)
}

fn simulate(inp: &str, is_keypad: bool, mult: usize, cmds: &mut Counter) {
    let (mut cur, shifts) = if is_keypad {
        (KEYPAD_INDICES[&b'A'], parse_code(inp))
    } else {
        (ROBOPAD_INDICES[&b'A'], extract_shifts(inp))
    };
    for shift in shifts {
        let (cmd, nxt) = process_command(is_keypad, cur, shift);
        *cmds.entry(cmd).or_default() += mult;
        cur = nxt;
    }
}

fn compute_weight(counter: &Counter) -> usize {
    counter.iter().map(|(cmd, count)| cmd.len() * count).sum()
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let reader = BufReader::new(File::open(input).unwrap());
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut cmds = Counter::new();
    let mut new_cmds = Counter::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let number: usize = line[..line.len() - 1].parse().unwrap();
        cmds.clear();
        simulate(&line, true, 1, &mut cmds);
        for step in 2..=STEPS {
            for (cmd, count) in cmds.drain() {
                simulate(&cmd, false, count, &mut new_cmds);
            }
            mem::swap(&mut cmds, &mut new_cmds);
            if step == 2 {
                ans1 += number * compute_weight(&cmds);
            }
        }
        ans2 += number * compute_weight(&cmds);
    }
    println!("ans1 = {ans1}");
    println!("ans2 = {ans2}");
}
