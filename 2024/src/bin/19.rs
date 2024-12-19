//  rustc 19.rs && ./19

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "19.in";

fn count_ways(s: &String, towels: &HashSet<String>, ways: &mut HashMap<String, i64>) -> i64 {
    match ways.get(s) {
        Some(ans) => return *ans,
        None => {} // fall through
    }
    let mut ans = towels.contains(s) as i64;
    let n = s.len();
    for r in 1..n {
        if towels.contains(&s[0..r]) {
            ans += count_ways(&s[r..n].to_string(), &towels, ways);
        }
    }
    ways.insert(s.to_owned(), ans);
    return ans;
}

fn solve(towels: &HashSet<String>, designs: &Vec<String>) -> (i32, i64) {
    let mut ans_part1 = 0;
    let mut ans_part2 = 0;
    for d in designs {
        let cnt = count_ways(d, &towels, &mut HashMap::new());
        ans_part1 += (cnt > 0) as i32;
        ans_part2 += cnt;
        // println!("design = {} , cnt = {}", *d, cnt);
    }
    return (ans_part1, ans_part2);
}

fn main() {
    let mut towels: HashSet<String> = HashSet::new();
    let mut designs: Vec<String> = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.contains(",") {
            towels = l.split(", ").map(|s| (*s).to_string()).collect();
        } else if l.is_empty() {
            continue;
        } else {
            designs.push(l);
        }
    }

    let p = solve(&towels, &designs);
    println!("ans_part1 = {}\nans_part2 = {}", p.0, p.1);
}
