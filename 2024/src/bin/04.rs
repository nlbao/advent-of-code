//  rustc 04.rs && ./04

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "4.in";

fn part1(words: &Vec<Vec<char>>) -> i64 {
    let xmas: Vec<char> = vec!['X', 'M', 'A', 'S'];
    let dx: Vec<i32> = vec![0, 0, -1, -1, -1, 1, 1, 1];
    let dy: Vec<i32> = vec![-1, 1, 0, -1, 1, 0, -1, 1];
    let (n_row, n_col) = (words.len(), words[0].len());
    let (n_row32, n_col32) = (n_row as i32, n_col as i32);
    let mut ans: i64 = 0;
    for i in 0..n_row {
        for j in 0..n_col {
            if words[i][j] != 'X' {
                continue;
            }
            for t in 0..dx.len() {
                let mut x = i as i32;
                let mut y = j as i32;
                for k in 1..4 {
                    x += dx[t];
                    y += dy[t];
                    if x < 0 || y < 0 || x >= n_row32 || y >= n_col32 {
                        break;
                    }
                    if words[x as usize][y as usize] != xmas[k] {
                        break;
                    }
                    if k == 3 {
                        ans += 1;
                    }
                }
            }
        }
    }
    return ans;
}

fn part2(words: &Vec<Vec<char>>) -> i64 {
    let mut ans: i64 = 0;
    let (n_row, n_col) = (words.len(), words[0].len());
    for i in 1..n_row - 1 {
        for j in 1..n_col - 1 {
            if words[i][j] != 'A' {
                continue;
            }
            let diag0 = [words[i - 1][j - 1], words[i + 1][j + 1]];
            let diag1 = [words[i - 1][j + 1], words[i + 1][j - 1]];
            if diag0.contains(&'M')
                && diag0.contains(&'S')
                && diag1.contains(&'M')
                && diag1.contains(&'S')
            {
                ans += 1;
            }
        }
    }
    return ans;
}

fn main() {
    let mut words: Vec<Vec<char>> = Vec::new();
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        words.push(l.chars().collect());
    }

    println!("{}", part1(&words));
    println!("{}", part2(&words));
}
