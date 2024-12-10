//  rustc 10.rs && ./10

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "10.in";

fn char2int(ch: char) -> i32 {
    return (ch as i32) - ('0' as i32);
}

fn get_adj_cells(x: usize, y: usize, n_row: usize, n_col: usize) -> Vec<(usize, usize)> {
    let dx: Vec<i32> = vec![0, 0, -1, 1];
    let dy: Vec<i32> = vec![-1, 1, 0, 0];
    let (n_row32, n_col32) = (n_row as i32, n_col as i32);
    let (x_32, y_32) = (x as i32, y as i32);
    let mut ans: Vec<(usize, usize)> = vec![];
    for k in 0..4 {
        let i = x_32 + dx[k];
        let j = y_32 + dy[k];
        if i < 0 || j < 0 || i >= n_row32 || j >= n_col32 {
            continue;
        }
        ans.push((i as usize, j as usize));
    }
    return ans;
}

fn dfs(
    is_part1: bool,
    map: &Vec<Vec<char>>,
    marks: &mut Vec<Vec<i32>>,
    x: usize,
    y: usize,
    turn: i32,
) -> i64 {
    if is_part1 {
        if marks[x][y] == turn {
            return 0;
        }
        marks[x][y] = turn;
    }
    let next_val = char2int(map[x][y]) + 1;
    if next_val == 10 {
        return 1;
    }
    let mut ans = 0;
    for (xx, yy) in get_adj_cells(x, y, map.len(), map[0].len()) {
        if char2int(map[xx][yy]) == next_val {
            ans += dfs(is_part1, map, marks, xx, yy, turn);
        }
    }
    return ans;
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        map.push(l.chars().collect());
    }

    // solve it
    let (n_row, n_col) = (map.len(), map[0].len());
    let mut marks: Vec<Vec<i32>> = vec![vec![0; n_col]; n_row];
    let mut turn = 0;
    let mut ans_part1 = 0;
    let mut ans_part2 = 0;
    for i in 0..n_row {
        for j in 0..n_col {
            if map[i][j] == '0' {
                turn += 1;
                ans_part1 += dfs(true, &map, &mut marks, i, j, turn);
                ans_part2 += dfs(false, &map, &mut marks, i, j, -1);
            }
        }
    }

    println!("{}", ans_part1);
    println!("{}", ans_part2);
}
