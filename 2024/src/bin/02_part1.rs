//  rustc 02_part1.rs && ./02_part1

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "2.in";

fn diff_is_ok(x: i32, y: i32) -> bool {
    let diff = (x - y).abs();
    return diff >= 1 && diff <= 3;
}

fn main() {
    let mut ans: i32 = 0;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let v: Vec<&str> = l.split(" ").collect();
        if v.len() < 2 {
            ans += 1;
            continue;
        }
        let mut x0: i32 = v[0].parse().unwrap();
        let mut x1: i32 = v[1].parse().unwrap();
        if !diff_is_ok(x0, x1) {
            continue;
        }
        let mut ok: bool = true;
        let is_increasing = x0 < x1;
        for i in 2..v.len() {
            x0 = x1;
            x1 = v[i].parse().unwrap();
            if (is_increasing && x0 >= x1) || (!is_increasing && x0 <= x1) || (!diff_is_ok(x0, x1))
            {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}
