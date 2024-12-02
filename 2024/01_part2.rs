//  rustc 01_part2.rs && ./01_part2

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "1.in";

fn main() {
    let mut vec1: Vec<i32> = vec![0; 0];
    let mut freqs = HashMap::<i32, i32>::new();

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned(); // https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211/2
        let v: Vec<&str> = l.split("   ").collect();
        vec1.push(v[0].parse().unwrap());
        let f = freqs.entry(v[1].parse().unwrap()).or_insert(0);
        *f += 1;
    }

    let mut ans: i64 = 0;
    for i in 0..vec1.len() {
        let x = vec1[i];
        match freqs.get(&x) {
            Some(f) => ans += i64::from(x) * i64::from(*f),
            None => {}
        }
    }
    println!("{}", ans);
}
