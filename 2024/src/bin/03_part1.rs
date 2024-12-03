//  cargo run --bin 03_part1

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "3.in";

fn main() {
    let mut ans: i64 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        for (_, [num1, num2]) in re.captures_iter(&l).map(|c| c.extract()) {
            ans += (num1.parse::<i64>().unwrap()) * (num2.parse::<i64>().unwrap());
        }
    }
    println!("{}", ans);
}
