//  rustc 09.rs && ./09

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "9.in";

fn get_layout(digits: &Vec<i32>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut id: i32 = 0;
    for i in 0..digits.len() {
        let mut x = -1;
        if i & 1 == 0 {
            x = id;
            id += 1;
        }
        for _ in 0..digits[i] {
            v.push(x);
        }
    }
    return v;
}

fn part1(digits: &Vec<i32>) -> i64 {
    let mut v = get_layout(digits);
    let mut ans = 0;
    let mut r = v.len() - 1;
    for i in 0..v.len() {
        while i <= r && v[r] == -1 {
            r -= 1;
        }
        if i > r {
            break;
        }
        if v[i] == -1 {
            assert!(i < r);
            v[i] = v[r];
            v[r] = -1;
            r -= 1;
        }
        ans += (v[i] as i64) * (i as i64);
    }
    return ans;
}

fn checksum(offset: i32, file_len: i32, file_id: i32) -> i64 {
    // ans = sum [(offset + i) * file_id] for i in 0..file_len.
    let len64 = file_len as i64;
    return (file_id as i64) * (len64 * (offset as i64) + (len64 * (len64 - 1)) / 2);
}

fn part2(digits: &Vec<i32>) -> i64 {
    // let mut v = get_layout(digits);
    let mut block_lens = vec![];
    let mut block_offsets = vec![];
    let mut file_lens = vec![];
    let mut file_offsets = vec![];
    let mut file_ids = vec![];
    let mut id: i32 = 0;
    let mut offset: i32 = 0;
    for i in 0..digits.len() {
        if i & 1 == 0 {
            file_lens.push(digits[i]);
            file_offsets.push(offset);
            file_ids.push(id);
            id += 1;
        } else {
            block_lens.push(digits[i]);
            block_offsets.push(offset);
        }
        offset += digits[i];
    }

    let mut ans = 0;
    for r in (0..file_ids.len()).rev() {
        let (offset, len, id) = (file_offsets[r], file_lens[r], file_ids[r]);
        let mut moved = false;
        for i in 0..block_lens.len() {
            if block_offsets[i] > offset {
                break;
            }
            if block_lens[i] >= len {
                ans += checksum(block_offsets[i], len, id);
                block_offsets[i] += len;
                block_lens[i] -= len;
                moved = true;
                break;
            }
        }
        if !moved {
            ans += checksum(offset, len, id);
        }
    }
    return ans;
}

fn main() {
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let v: Vec<i32> = l.chars().map(|x| x as i32 - '0' as i32).collect();
        println!("{}", part1(&v));
        println!("{}", part2(&v));
        break;
    }
}
