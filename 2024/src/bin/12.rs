//  rustc 12.rs && ./12

use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "12.in";
const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;
const TOP_LEFT: usize = 4;
const TOP_RIGHT: usize = 5;
const BOTTOM_LEFT: usize = 6;
const BOTTOM_RIGHT: usize = 7;

fn get_adj_cells(x: usize, y: usize, n_row: usize, n_col: usize) -> Vec<(usize, usize, usize)> {
    let dx: Vec<i32> = vec![0, 0, -1, 1, -1, -1, 1, 1];
    let dy: Vec<i32> = vec![-1, 1, 0, 0, -1, 1, -1, 1];
    let (n_row32, n_col32) = (n_row as i32, n_col as i32);
    let (x_32, y_32) = (x as i32, y as i32);
    let mut ans: Vec<(usize, usize, usize)> = vec![];
    for k in 0..dx.len() {
        let i = x_32 + dx[k];
        let j = y_32 + dy[k];
        if i < 0 || j < 0 || i >= n_row32 || j >= n_col32 {
            continue;
        }
        ans.push((k, i as usize, j as usize));
    }
    return ans;
}

fn solve(map: &Vec<Vec<char>>) -> (i64, i64) {
    let mut ans_part1: i64 = 0;
    let mut ans_part2: i64 = 0;
    let (n_row, n_col) = (map.len(), map[0].len());
    const NOT_IN_QUEUE: i32 = 0;
    const IN_QUEUE: i32 = 1;
    const VISITED: i32 = 2;
    let mut state: Vec<Vec<i32>> = vec![vec![NOT_IN_QUEUE; n_col]; n_row];
    for x in 0..n_row {
        for y in 0..n_col {
            if state[x][y] == VISITED {
                continue;
            }
            let plant = map[x][y];
            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = 0;
            let mut q = VecDeque::new();
            q.push_back((x, y));
            while !q.is_empty() {
                let (i, j) = q.pop_front().unwrap();
                state[i][j] = VISITED;
                let mut edges = 4;
                let mut same_plant: Vec<bool> = vec![false; 8];
                for (dir, ii, jj) in get_adj_cells(i, j, n_row, n_col) {
                    if map[ii][jj] != plant {
                        continue;
                    }
                    same_plant[dir] = true;
                    if dir >= 4 || state[ii][jj] == IN_QUEUE {
                        continue;
                    }
                    if state[ii][jj] == VISITED {
                        edges -= 2;
                    } else {
                        q.push_back((ii, jj));
                        state[ii][jj] = IN_QUEUE;
                    }
                }
                perimeter += edges;
                area += 1;
                sides += ((!same_plant[LEFT] && (!same_plant[TOP] || same_plant[TOP_LEFT])) as i64) // count left side
                    + ((!same_plant[TOP] && (!same_plant[RIGHT] || same_plant[TOP_RIGHT])) as i64) // count top side
                    + ((!same_plant[RIGHT] && (!same_plant[BOTTOM] || same_plant[BOTTOM_RIGHT])) // count right side
                        as i64)
                    + ((!same_plant[BOTTOM] && (!same_plant[LEFT] || same_plant[BOTTOM_LEFT])) // count bottom side
                        as i64);
            }
            ans_part1 += area * perimeter;
            ans_part2 += area * sides;
            // println!("{} : {} * {} = {}", plant, area, sides, area * sides);
        }
    }
    return (ans_part1, ans_part2);
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        map.push(l.chars().collect());
    }

    let (ans1, ans2) = solve(&map);
    println!("{}\n{}", ans1, ans2);
}
