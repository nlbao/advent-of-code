//  rustc 25.rs && ./25

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "25.in";

fn is_lock(layout: &Vec<Vec<char>>) -> bool {
    return layout[0].iter().all(|c| *c == '#');
}

fn to_nums(layout: &Vec<Vec<char>>) -> Vec<usize> {
    let (n_row, n_col) = (layout.len(), layout[0].len());
    let mut ans = vec![];
    for y in 0..n_col {
        let cnt = (0..n_row)
            .into_iter()
            .filter(|x| (layout[*x][y] == '#'))
            .count();
        ans.push(cnt);
    }
    return ans;
}

fn add_layout(layout: &Vec<Vec<char>>, locks: &mut Vec<Vec<usize>>, keys: &mut Vec<Vec<usize>>) {
    if is_lock(&layout) {
        locks.push(to_nums(&layout));
    } else {
        keys.push(to_nums(&layout));
    }
}

fn is_key_of_lock(lock: &Vec<usize>, keys: &Vec<usize>) -> bool {
    return (0..lock.len()).into_iter().all(|i| lock[i] + keys[i] <= 7);
}

fn solve(locks: &Vec<Vec<usize>>, keys: &Vec<Vec<usize>>) -> i32 {
    let mut ans = 0;
    for lock in locks {
        for key in keys {
            if is_key_of_lock(lock, key) {
                ans += 1;
            }
        }
    }
    return ans;
}

fn main() {
    let mut layout = vec![];
    let mut locks = vec![];
    let mut keys = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.is_empty() {
            add_layout(&layout, &mut locks, &mut keys);
            layout = vec![];
        } else {
            layout.push(l.chars().collect());
        }
    }
    if !layout.is_empty() {
        add_layout(&layout, &mut locks, &mut keys);
    }
    println!("{}", solve(&locks, &keys));
}
