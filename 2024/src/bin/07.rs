//  rustc 07.rs && ./07

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "7.in";

fn concat(a: i64, b: i64) -> i64 {
    let s = a.to_string() + &b.to_string();
    match s.parse() {
        Ok(t) => return t,
        Err(_) => return -1, // overflow.
    }
}

fn solvable(
    enable_concatenation: bool,
    target: i64,
    nums: &Vec<i64>,
    i: usize,
    current: i64,
) -> bool {
    if current > target {
        return false;
    }
    if i == nums.len() {
        return current == target;
    }
    if enable_concatenation {
        let c = concat(current, nums[i]);
        if c > -1 && solvable(enable_concatenation, target, nums, i + 1, c) {
            return true;
        }
    }
    if target / current >= nums[i]
        && solvable(enable_concatenation, target, nums, i + 1, current * nums[i])
    {
        return true;
    }
    return solvable(enable_concatenation, target, nums, i + 1, current + nums[i]);
}

fn main() {
    let mut ans_part1 = 0;
    let mut ans_part2 = 0;
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let equation: Vec<&str> = l.split(": ").collect();
        let target: i64 = equation[0].parse().unwrap();
        let nums: Vec<i64> = equation[1].split(" ").map(|s| s.parse().unwrap()).collect();
        if solvable(false, target, &nums, 1, nums[0]) {
            ans_part1 += target;
        } else if solvable(true, target, &nums, 1, nums[0]) {
            ans_part2 += target;
        }
    }
    ans_part2 += ans_part1;
    println!("{}", ans_part1);
    println!("{}", ans_part2);
}
