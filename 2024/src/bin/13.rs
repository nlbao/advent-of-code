//  cargo run --bin 13

use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "13.in";

fn solve((xa, ya): (i64, i64), (xb, yb): (i64, i64), (xprize, yprize): (i64, i64)) -> i64 {
    let ni = yb * xprize - xb * yprize;
    let di = xa * yb - xb * ya;
    if di == 0 || ni % di != 0 {
        return 0;
    }
    let i = ni / di;
    //
    let nk = yprize - ya * i;
    if yb == 0 || nk % yb != 0 {
        return 0;
    }
    let k = nk / yb;
    if i < 0 || k < 0 {
        return 0;
    }
    return 3 * i + k;
}

fn main() {
    let re_button = Regex::new(r".* X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = Regex::new(r".* X=(\d+), Y=(\d+)").unwrap();
    let part2_add = 10000000000000;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    let (mut xa, mut ya) = (0, 0);
    let (mut xb, mut yb) = (0, 0);
    let mut ans_1 = 0;
    let mut ans_2 = 0;
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.is_empty() {
            continue;
        }
        let is_prize = l.starts_with("Prize:");
        let iter = if is_prize {
            re_prize.captures_iter(&l).map(|c| c.extract()).next()
        } else {
            re_button.captures_iter(&l).map(|c| c.extract()).next()
        };
        let (_, [xs, ys]) = iter.unwrap();
        let (x, y): (i64, i64) = (xs.parse().unwrap(), ys.parse().unwrap());
        if is_prize {
            ans_1 += solve((xa, ya), (xb, yb), (x, y));
            ans_2 += solve((xa, ya), (xb, yb), (x + part2_add, y + part2_add));
        } else if l.starts_with("Button A:") {
            (xa, ya) = (x, y);
        } else {
            (xb, yb) = (x, y);
        }
    }

    println!("{}", ans_1);
    println!("{}", ans_2);
}
