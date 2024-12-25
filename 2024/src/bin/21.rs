//  rustc 21.rs && ./21

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "21.in";

const DX: &'static [i32] = &[0, 0, -1, 1];
const DY: &'static [i32] = &[-1, 1, 0, 0];

const NULL_BUTTON: char = '#';
const NULL_STATE: (usize, char, char, char) = (usize::MAX, '#', '#', '#');

// TODO: also consider different path options from NUM_KEYPAD.
const NUM_KEYPAD: &'static [&'static [char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &['#', '0', 'A'],
];

const DIR_KEYPAD: &'static [&'static [char]] = &[
    &['#', '^', 'A'], //
    &['<', 'v', '>'],
];

// dir buttons needed to move the pointer from cell s to cell t
// many options.
fn get_dir_buttons_to_cell(s: &char, t: &char) -> Vec<String> {
    let ans = match (s, t) {
        ('^', 'A') => Vec::from([">"]),
        ('^', 'v') => Vec::from(["v"]),
        ('^', '<') => Vec::from(["v<"]),
        ('^', '>') => Vec::from(["v>", ">v"]),
        //
        ('A', '^') => Vec::from(["<"]),
        ('A', '>') => Vec::from(["v"]),
        ('A', 'v') => Vec::from(["v<", "<v"]),
        ('A', '<') => Vec::from(["v<<", "<v<"]),
        //
        ('>', 'A') => Vec::from(["^"]),
        ('>', 'v') => Vec::from(["<"]),
        ('>', '^') => Vec::from(["^<", "<^"]),
        ('>', '<') => Vec::from(["<<"]),
        //
        ('v', '<') => Vec::from(["<"]),
        ('v', '>') => Vec::from([">"]),
        ('v', '^') => Vec::from(["^"]),
        ('v', 'A') => Vec::from(["^>", ">^"]),
        //
        ('<', 'v') => Vec::from([">"]),
        ('<', '>') => Vec::from([">>"]),
        ('<', '^') => Vec::from([">^"]),
        ('<', 'A') => Vec::from([">^>", ">>^"]),
        _ => unreachable!(),
    };
    return ans.into_iter().map(|x| x.to_string()).collect();
}

fn get_adj_dir_buttons(b: char) -> Vec<char> {
    return match b {
        '^' => Vec::from(['A', 'v']),
        'A' => Vec::from(['^', '>']),
        '<' => Vec::from(['v']),
        'v' => Vec::from(['<', '^', '>']),
        '>' => Vec::from(['v', 'A']),
        _ => unreachable!(),
    };
}

fn next_button(b: char, dir: char, keypad: &'static [&'static [char]]) -> char {
    let (n_row, n_col) = (keypad.len(), keypad[0].len());
    let (mut xb, mut yb) = (i32::MAX, i32::MAX);
    for x in 0..n_row {
        for y in 0..n_col {
            if keypad[x][y] == b {
                (xb, yb) = (x as i32, y as i32);
                break;
            }
        }
        if xb < i32::MAX {
            break;
        }
    }
    let k = match dir {
        '<' => 0,
        '>' => 1,
        '^' => 2,
        'v' => 3,
        _ => unreachable!(),
    };
    xb += DX[k];
    yb += DY[k];
    if xb < 0 || yb < 0 || xb >= n_row as i32 || yb >= n_col as i32 {
        return NULL_BUTTON;
    }
    return keypad[xb as usize][yb as usize];
}

fn next_dir_button(b: char, dir: char) -> char {
    return next_button(b, dir, DIR_KEYPAD);
}

fn next_numeric_button(b: char, dir: char) -> char {
    return next_button(b, dir, NUM_KEYPAD);
}

fn get_press_button_state(
    state: (usize, char, char, char),
    code_val: char,
) -> (usize, char, char, char) {
    let (code_i, b1, b2, b3) = state;
    // press b1
    if b1 != 'A' {
        let new_b2 = next_dir_button(b2, b1);
        if new_b2 == NULL_BUTTON {
            return NULL_STATE;
        }
        return (code_i, b1, new_b2, b3);
    }
    // press b2
    if b2 != 'A' {
        let new_b3 = next_numeric_button(b3, b2);
        if new_b3 == NULL_BUTTON {
            return NULL_STATE;
        }
        return (code_i, b1, b2, new_b3);
    }
    // press b3
    if b3 == code_val {
        return (code_i + 1, b1, b2, b3);
    }
    return NULL_STATE;
}

fn shortest_path(code: &Vec<char>) -> (i64, String) {
    let mut cost = HashMap::new();
    let mut heap = BTreeSet::new();
    let mut paths = HashMap::new();
    // state = (code_i, robot1_button, robot2_button, robot3_button)
    cost.insert((0, 'A', 'A', 'A'), 0);
    heap.insert((0, (0, 'A', 'A', 'A')));
    paths.insert((0, 'A', 'A', 'A'), "".to_string());
    while !heap.is_empty() {
        let (c, current_state) = heap.pop_first().unwrap();
        let (code_i, b1, b2, b3) = current_state;
        let crr_path = paths.get(&current_state).unwrap().to_owned();
        if code_i >= code.len() {
            return (c, crr_path);
        }
        let new_cost = c + 1;
        let mut next_states = vec![];
        // press the button
        let press_button_state = get_press_button_state(current_state, code[code_i]);
        if press_button_state != NULL_STATE {
            next_states.push(press_button_state);
        }
        // move to an adj button
        for new_b1 in get_adj_dir_buttons(b1) {
            next_states.push((code_i, new_b1, b2, b3));
        }
        // update next states
        for state in next_states {
            let state_cost = *cost.get(&state).unwrap_or(&i64::MAX);
            if new_cost >= state_cost {
                continue;
            }
            if state_cost < i64::MAX {
                heap.remove(&(state_cost, state));
            }
            heap.insert((new_cost, state));
            cost.insert(state, new_cost);
            if state.1 == current_state.1 {
                // the first button was pressed.
                let new_path = format!("{}{}", crr_path.clone(), state.1);
                paths.insert(state, new_path);
            } else {
                paths.insert(state, crr_path.to_owned());
            }
        }
    }
    unreachable!();
}

fn numeric_val(code: &Vec<char>) -> i64 {
    let mut val = 0;
    for c in code {
        let x = *c as i64;
        if x >= ('0' as i64) && x <= ('9' as i64) {
            val = val * 10 + (x - '0' as i64);
        }
    }
    return val;
}

// assumption: always starts at "A" (not included in s), and ends at "A" (included)
fn part2(level: usize, s: &String, cache: &mut HashMap<(usize, String), i64>) -> i64 {
    println!("part2: level = {}, s = {}", level, s);
    let n = s.len();
    if level == 0 || n == 1 {
        return n as i64;
    }
    let state = (level, s.clone());
    match cache.get(&state) {
        Some(v) => return *v,
        None => {} // fall through
    }
    let mut pre: char = 'A';
    let mut ans = 0;
    let a_str = "A".to_string();
    for c in s.chars() {
        let mut next_level_s_options = vec![];
        if c != pre {
            let button_options = get_dir_buttons_to_cell(&pre, &c);
            for b in button_options {
                next_level_s_options.push(b + &a_str);
            }
            pre = c;
        } else {
            next_level_s_options.push(a_str.clone());
        }
        ans += next_level_s_options
            .iter()
            .map(|next_level_s| part2(level - 1, &next_level_s, cache))
            .min() // choose the shortest option
            .unwrap();
    }
    cache.insert(state, ans);
    return ans;
}

fn main() {
    assert!(next_dir_button('A', '>') == NULL_BUTTON);
    assert!(next_dir_button('A', '^') == NULL_BUTTON);

    let mut ans_part1 = 0;
    let mut ans_part2 = 0;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let code: Vec<char> = l.chars().collect();
        let val = numeric_val(&code);
        let (dist, path) = shortest_path(&code);
        let a2_level1 = part2(1, &path, &mut HashMap::new());
        // let a2 = part2(24, &path, &mut cache);
        let a2 = part2(24, &path, &mut HashMap::new());
        println!(
            "code = {:?}   val = {}, shortest_path = {}, path = {},     a2_level1 = {},     a2 = {}",
            code, val, dist, path, a2_level1, a2,
        );
        assert!(a2_level1 == dist);
        ans_part1 += val * dist;
        ans_part2 += val * a2;
    }
    println!("ans_part1 = {}\n", ans_part1);
    println!("ans_part2 = {}\n", ans_part2);
    // no min()
    // 2429186610257336 is too high (level = 24)
    // 894450181544366 is too high (level = 23)

    // with min()
    // 337263686512582 is too high (level = 24)
    // 135576559312082 is not the right answer (level = 23)
}
