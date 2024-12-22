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

fn shortest_path(code: &Vec<char>) -> i64 {
    let mut cost = HashMap::new();
    let mut heap = BTreeSet::new();
    // state = (code_i, robot1_button, robot2_button, robot3_button)
    cost.insert((0, 'A', 'A', 'A'), 0);
    heap.insert((0, (0, 'A', 'A', 'A')));
    while !heap.is_empty() {
        let (c, current_state) = heap.pop_first().unwrap();
        let (code_i, b1, b2, b3) = current_state;
        if code_i >= code.len() {
            return c;
        }
        let new_cost = c + 1;
        let mut next_states = vec![];
        // press the button
        let press_button_state = get_press_button_state(current_state, code[code_i]);
        if press_button_state != NULL_STATE {
            next_states.push(press_button_state);
        }
        // let
        // move to an adj button
        for new_b1 in get_adj_dir_buttons(b1) {
            next_states.push((code_i, new_b1, b2, b3));
        }
        // update next states
        for state in next_states {
            if cost.contains_key(&state) {
                let state_cost = *cost.get(&state).unwrap();
                if new_cost < state_cost {
                    heap.remove(&(state_cost, state));
                }
            }
            heap.insert((new_cost, state));
            cost.insert(state, new_cost);
        }
    }
    unreachable!();
}

fn solve(code: &Vec<char>) -> i64 {
    let mut val = 0;
    for c in code {
        let x = *c as i64;
        if x >= ('0' as i64) && x <= ('9' as i64) {
            val = val * 10 + (x - '0' as i64);
        }
    }
    let dist = shortest_path(code);
    println!("code = {:?}   val = {}, shortest_path = {}", code, val, dist);
    return val * dist;
}

fn main() {
    let mut ans_part1 = 0;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let code = l.chars().collect();
        ans_part1 += solve(&code);
    }
    println!("ans_part1 = {}\n", ans_part1);
}
