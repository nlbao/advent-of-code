//  rustc 18.rs && ./18

use std::collections::BTreeSet;
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "18.in";

const DX: &'static [i32] = &[0, 0, -1, 1];
const DY: &'static [i32] = &[-1, 1, 0, 0];

fn part1((xt, yt): (usize, usize), bytes: &Vec<(usize, usize)>, n_byte: usize) -> i32 {
    let (n_row, n_col) = (xt + 1, yt + 1);
    let (n_row_32, n_col_32) = (n_row as i32, n_col as i32);
    let mut ok: Vec<Vec<bool>> = vec![vec![true; n_col]; n_row];
    for i in 0..n_byte {
        ok[bytes[i].0][bytes[i].1] = false;
    }
    let mut heap = BTreeSet::new();
    let mut f: Vec<Vec<i32>> = vec![vec![i32::MAX; n_col]; n_row];
    f[0][0] = 0;
    heap.insert((0, 0, 0));
    while !heap.is_empty() {
        let (cost, x, y) = heap.pop_first().unwrap();
        if (x, y) == (xt, yt) {
            return cost;
        }
        for k in 0..4 {
            let i = (x as i32) + DX[k];
            let j = (y as i32) + DY[k];
            if i < 0 || j < 0 || i >= n_row_32 || j >= n_col_32 {
                continue;
            }
            let (xx, yy) = (i as usize, j as usize);
            if !ok[xx][yy] || f[xx][yy] <= cost + 1 {
                continue;
            }
            if f[xx][yy] < i32::MAX {
                heap.remove(&(f[xx][yy], xx, yy));
            }
            f[xx][yy] = cost + 1;
            heap.insert((cost + 1, xx, yy));
        }
    }
    return i32::MAX;
}

fn main() {
    let (mut xt, mut yt, mut n_byte) = (0, 0, 0);
    let mut bytes: Vec<(usize, usize)> = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.contains(",") {
            let v: Vec<usize> = l.split(",").map(|c| (*c).parse().unwrap()).collect();
            bytes.push((v[1], v[0]));
        } else {
            let v: Vec<usize> = l.split(" ").map(|c| (*c).parse().unwrap()).collect();
            (xt, yt) = (v[1], v[0]);
            n_byte = v[2];
        }
    }

    println!("ans_part1 = {}", part1((xt, yt), &bytes, n_byte));

    // part 2
    for i in n_byte + 1..bytes.len() {
        let cost = part1((xt, yt), &bytes, i + 1);
        if cost == i32::MAX {
            println!("ans_part2 = {},{}", bytes[i].1, bytes[i].0);
            break;
        }
    }
}
