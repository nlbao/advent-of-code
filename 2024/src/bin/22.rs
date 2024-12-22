//  rustc 22.rs && ./22

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "22.in";

const MOD: i64 = 16777216;

fn next_secret(x: i64) -> i64 {
    let y = ((x * 64) ^ x) % MOD;
    let z = ((y / 32) ^ y) % MOD;
    return ((z * 2048) ^ z) % MOD;
}

fn part2(seq2score: &mut HashMap<String, i64>, v: &Vec<(i64, i64)>) {
    let mut visited: HashSet<String> = HashSet::new();
    for l in 0..v.len() - 3 {
        let r = l + 4;
        let sub4: Vec<String> = (l..r).map(|i| v[i].1.to_string()).collect();
        let s = sub4.join(",");
        if visited.contains(&s) {
            continue;
        }
        visited.insert(s.to_owned());
        *seq2score.entry(s).or_insert(0) += v[r - 1].0; // + price
    }
}

fn solve(inits: &Vec<i64>) -> (i64, i64) {
    let mut ans_part1 = 0;
    let mut seq2score = HashMap::new();
    for x in inits {
        let mut next2k = *x;
        let mut pre_price = next2k % 10;
        let mut v = vec![];
        for _ in 0..2000 {
            next2k = next_secret(next2k);
            let price = next2k % 10;
            v.push((price, price - pre_price));
            pre_price = price;
        }
        ans_part1 += next2k;
        part2(&mut seq2score, &v);
    }
    return (ans_part1, *seq2score.values().max().unwrap());
}

fn main() {
    let mut inits: Vec<i64> = Vec::new();
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        inits.push(l.parse().unwrap());
    }
    println!("ans = {:?}\n", solve(&inits));
}
