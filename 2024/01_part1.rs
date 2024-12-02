//  rustc 01_part1.rs && ./01_part1

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "1.in";

fn main() {
    let mut vec1: Vec<i32> = vec![0; 0];
    let mut vec2: Vec<i32> = vec![0; 0];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned(); // https://users.rust-lang.org/t/creates-a-temporary-which-is-freed-while-still-in-use-again/29211/2
        let v: Vec<&str> = l.split("   ").collect();
        vec1.push(v[0].parse().unwrap());
        vec2.push(v[1].parse().unwrap());
    }
    assert!(vec1.len() == vec2.len());

    vec1.sort();
    vec2.sort();
    let mut ans = 0;
    for i in 0..vec1.len() {
        ans += (vec1[i] - vec2[i]).abs();
    }
    println!("{}", ans);
}
