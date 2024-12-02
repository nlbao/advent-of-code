//  rustc 02_part2.rs && ./02_part2

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "2.in";

fn diff_is_ok(x: i32, y: i32) -> bool {
    let diff = (x - y).abs();
    return diff >= 1 && diff <= 3;
}

fn is_safe_report(v: &Vec<i32>) -> bool {
    if v.len() < 2 {
        return false;
    }
    let mut x0 = v[0];
    let mut x1 = v[1];
    if !diff_is_ok(x0, x1) {
        return false;
    }
    let is_increasing = x0 < x1;
    for i in 2..v.len() {
        x0 = x1;
        x1 = v[i];
        if (is_increasing && x0 >= x1) || (!is_increasing && x0 <= x1) || (!diff_is_ok(x0, x1)) {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut ans: i32 = 0;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let v: Vec<i32> = l.split(" ").map(|s| s.parse().unwrap()).collect();
        if is_safe_report(&v) {
            ans += 1;
            continue;
        }
        for i in 0..v.len() {
            // creates a new_v without v[i].
            let mut new_v: Vec<i32> = vec![];
            for k in 0..v.len() {
                if k != i {
                    new_v.push(v[k]);
                }
            }
            if is_safe_report(&new_v) {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}
