//  rustc 21.rs && ./21

use std::cmp::min;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "21.in";

const DX: &'static [i32] = &[0, 0, -1, 1];
const DY: &'static [i32] = &[-1, 1, 0, 0];
const DIR_BUTTONS: &'static [char] = &['<', '>', '^', 'v'];

const NULL_XY: (i32, i32) = (i32::MAX, i32::MAX);
const NULL_BUTTON: char = '#';

const NUM_KEYPAD: &'static [&'static [char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &['#', '0', 'A'],
];

const _DIR_KEYPAD: &'static [&'static [char]] = &[
    &['#', '^', 'A'], //
    &['<', 'v', '>'],
];

// dir buttons OPTIONS needed to move the pointer from cell s to cell t.
fn get_dir_button_options_to_cell(s: &char, t: &char) -> Vec<String> {
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

fn find_num(x: i32, y: i32) -> char {
    if x < 0 || y < 0 || x >= NUM_KEYPAD.len() as i32 || y >= NUM_KEYPAD[0].len() as i32 {
        return NULL_BUTTON;
    }
    return NUM_KEYPAD[x as usize][y as usize];
}

fn find_num_xy(num: char) -> (i32, i32) {
    for x in 0..NUM_KEYPAD.len() {
        for y in 0..NUM_KEYPAD[0].len() {
            if NUM_KEYPAD[x][y] == num {
                return (x as i32, y as i32);
            }
        }
    }
    return NULL_XY;
}

// convert from a numeric code --> directional sequences.
fn code_to_direction_options(code: &Vec<char>) -> HashSet<String> {
    let mut ans = HashSet::new();
    let mut cost = HashMap::new();
    let mut heap = BTreeSet::new();
    let mut paths = HashMap::new();
    cost.insert((0, 'A'), 0);
    heap.insert((0, (0, 'A')));
    paths.insert((0, 'A'), HashSet::from(["".to_string()]));
    while !heap.is_empty() {
        let (c, current_state) = heap.pop_first().unwrap();
        let (code_i, num) = current_state;
        let crr_path_set = paths.get(&current_state).unwrap().to_owned();
        if code_i >= code.len() {
            for p in crr_path_set {
                ans.insert(p);
            }
            continue;
        }
        let (xnum, ynum) = find_num_xy(num);
        let new_cost = c + 1;
        let mut next_states = vec![];
        // press the current num button, by pressing "A" in the directional keypad.
        if num == code[code_i] {
            next_states.push(('A', (code_i + 1, num)));
        }
        // move to an adjacent num button, by pressing a non-A directional button.
        for k in 0..4 {
            let (x, y) = (xnum + DX[k], ynum + DY[k]);
            let new_num = find_num(x, y);
            if new_num != NULL_BUTTON {
                next_states.push((DIR_BUTTONS[k], (code_i, new_num)));
            }
        }
        // update next states
        for (pressed_button, state) in next_states {
            let state_cost = *cost.get(&state).unwrap_or(&i64::MAX);
            if new_cost > state_cost {
                continue;
            }
            let path_entry = paths.entry(state).or_insert(HashSet::new());
            if new_cost < state_cost {
                if state_cost < i64::MAX {
                    heap.remove(&(state_cost, state));
                    path_entry.clear();
                }
                heap.insert((new_cost, state));
                cost.insert(state, new_cost);
            }
            for p in &crr_path_set {
                path_entry.insert(format!("{}{}", p, pressed_button));
            }
        }
    }
    return ans;
}

// assumption: always starts at "A" (not included in s), and ends at "A" (included)
fn shortest_dirs(level: usize, s: &String, cache: &mut HashMap<(usize, String), i64>) -> i64 {
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
            for b in get_dir_button_options_to_cell(&pre, &c) {
                next_level_s_options.push(b + &a_str);
            }
            pre = c;
        } else {
            next_level_s_options.push(a_str.clone());
        }
        ans += next_level_s_options
            .iter()
            .map(|next_level_s| shortest_dirs(level - 1, &next_level_s, cache))
            .min() // choose the shortest option
            .unwrap();
    }
    cache.insert(state, ans);
    return ans;
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

fn main() {
    let mut ans_part1 = 0;
    let mut ans_part2 = 0;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let code: Vec<char> = l.chars().collect();
        let val = numeric_val(&code);
        let (mut a1, mut a2) = (i64::MAX, i64::MAX);
        let mut cache1 = HashMap::new();
        let mut cache2 = HashMap::new();
        for path in code_to_direction_options(&code) {
            a1 = min(a1, shortest_dirs(2, &path, &mut cache1));
            a2 = min(a2, shortest_dirs(25, &path, &mut cache2));
        }
        println!("code = {} val = {}, a1 = {}, a2 = {}", l, val, a1, a2);
        ans_part1 += val * a1;
        ans_part2 += val * a2;
    }
    println!("\nans_part1 = {}", ans_part1);
    println!("\nans_part2 = {}", ans_part2);
}
