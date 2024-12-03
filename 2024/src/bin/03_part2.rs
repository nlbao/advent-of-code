//  cargo run --bin 03_part2

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "3.in";
const DO: &str = "do()";
const DO_NOT: &str = "don't()";

fn sum_product(s: &str) -> i64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut ans: i64 = 0;
    for (_, [num1, num2]) in re.captures_iter(&s).map(|c| c.extract()) {
        ans += (num1.parse::<i64>().unwrap()) * (num2.parse::<i64>().unwrap());
    }
    return ans;
}

fn main() {
    let mut ans: i64 = 0;
    let mut enabled = true;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut l = line.unwrap().to_owned();
        while l.len() > 0 {
            if enabled {
                match l.split_once(DO_NOT) {
                    Some((before, after)) => {
                        ans += sum_product(before);
                        l = after.to_owned();
                        enabled = false;
                    }
                    None => {
                        ans += sum_product(&l);
                        break;
                    }
                }
            } else {
                match l.split_once(DO) {
                    Some((_, after)) => {
                        l = after.to_owned();
                        enabled = true;
                    }
                    None => break,
                }
            }
        }
    }
    println!("{}", ans);
}
