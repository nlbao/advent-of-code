//  rustc 11.rs && ./11

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "11.in";

fn dfs(stone: i64, blink_times: usize, sol: &mut HashMap<(i64, usize), i64>) -> i64 {
    if blink_times == 0 {
        return 1;
    }
    let key = (stone, blink_times);
    match sol.get(&key) {
        Some(val) => return *val,
        None => {
            let ans;
            if stone == 0 {
                ans = dfs(1, blink_times - 1, sol);
            } else {
                let s = stone.to_string();
                let n = s.len();
                if n & 1 == 1 {
                    ans = dfs(stone * 2024, blink_times - 1, sol);
                } else {
                    let v = s.split_at(n >> 1);
                    ans = dfs(v.0.parse().unwrap(), blink_times - 1, sol)
                        + dfs(v.1.parse().unwrap(), blink_times - 1, sol);
                }
            }
            sol.insert((stone, blink_times), ans);
            return ans;
        }
    }
}

fn solve(stones: &Vec<i64>, blink_times: usize) -> i64 {
    let mut sol: HashMap<(i64, usize), i64> = HashMap::new();
    let mut ans: i64 = 0;
    for x in stones {
        ans += dfs(*x, blink_times, &mut sol);
    }
    return ans;
}

fn main() {
    let mut stones: Vec<i64> = Vec::new();
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        stones = l.split(" ").map(|x| x.parse().unwrap()).collect();
    }

    println!("{}", solve(&stones, 25));
    println!("{}", solve(&stones, 75));
}
