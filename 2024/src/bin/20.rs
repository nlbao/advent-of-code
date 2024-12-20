//  rustc 20.rs && ./20

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "20.in";

const DX: &'static [i32] = &[0, 0, -1, 1];
const DY: &'static [i32] = &[-1, 1, 0, 0];
const NULL_XY: (usize, usize) = (usize::MAX, usize::MAX);

fn get_adj_cell(x: usize, y: usize, dir: usize, n_row: usize, n_col: usize) -> (usize, usize) {
    let i = (x as i32) + DX[dir];
    let j = (y as i32) + DY[dir];
    if i < 0 || j < 0 || i >= (n_row as i32) || j >= (n_col as i32) {
        return NULL_XY;
    }
    return (i as usize, j as usize);
}

fn shortest_path(map: &Vec<Vec<char>>, (sx, sy): (usize, usize)) -> Vec<Vec<i32>> {
    let (n_row, n_col) = (map.len(), map[0].len());
    let mut cost: Vec<Vec<i32>> = vec![vec![i32::MAX; n_col]; n_row];
    let mut heap = BTreeSet::new();
    cost[sx][sy] = 0;
    heap.insert((0, (sx, sy)));
    while !heap.is_empty() {
        let (c, (x, y)) = heap.pop_first().unwrap();
        // update next_cells' cost
        for k in 0..4 {
            let (xx, yy) = get_adj_cell(x, y, k, n_row, n_col);
            if (xx, yy) == NULL_XY || map[xx][yy] == '#' {
                continue;
            }
            let new_cost = c + 1;
            if new_cost < cost[xx][yy] {
                heap.remove(&(cost[xx][yy], (xx, yy)));
                cost[xx][yy] = new_cost;
                heap.insert((new_cost, (xx, yy)));
            }
        }
    }
    return cost;
}

fn solve(min_cost: i32, s2e_cost: &Vec<Vec<i32>>, e2s_cost: &Vec<Vec<i32>>, limit: i32) -> i32 {
    let mut savings: HashMap<i32, i32> = HashMap::new();
    let (n_row, n_col) = (s2e_cost.len(), s2e_cost[0].len());
    let (n_row_32, n_col_32) = (n_row as i32, n_col as i32);
    for x in 0..n_row {
        for y in 0..n_col {
            let cost0 = s2e_cost[x][y];
            if cost0 == i32::MAX {
                continue;
            }
            let (x_32, y_32) = (x as i32, y as i32);
            for x2_32 in x_32 - limit..x_32 + limit + 1 {
                if x2_32 < 0 || x2_32 >= n_row_32 {
                    continue;
                }
                for y2_32 in y_32 - limit..y_32 + limit + 1 {
                    let cheat_time = (x2_32 - x_32).abs() + (y2_32 - y_32).abs();
                    if y2_32 < 0 || y2_32 >= n_col_32 || cheat_time > limit {
                        continue;
                    }
                    let cost2 = e2s_cost[x2_32 as usize][y2_32 as usize];
                    if cost2 == i32::MAX {
                        continue;
                    }
                    let cost = cost0 + cheat_time + cost2;
                    if cost < min_cost {
                        let s = min_cost - cost;
                        *savings.entry(s).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    // let mut v: Vec<(i32, i32)> = savings.iter().map(|(s, cnt)| (*s, *cnt)).collect();
    // v.sort();
    // for (s, cnt) in v {
    //     println!("There are {} cheats that save {} picoseconds.", cnt, s);
    // }
    return savings
        .iter()
        .filter(|(s, _)| **s >= 100)
        .map(|(_, cnt)| *cnt)
        .sum();
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        map.push(l.chars().collect());
    }

    let (mut sx, mut sy) = (0, 0);
    let (mut ex, mut ey) = (0, 0);
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 'S' {
                (sx, sy) = (x, y);
            } else if map[x][y] == 'E' {
                (ex, ey) = (x, y);
            }
        }
    }

    let s2e_cost = shortest_path(&map, (sx, sy));
    let e2s_cost = shortest_path(&map, (ex, ey));
    let min_cost = s2e_cost[ex][ey];
    println!("shortest_path = {}\n", min_cost);
    println!("ans_part1 = {}\n", solve(min_cost, &s2e_cost, &e2s_cost, 2));
    println!("ans_part2 = {}", solve(min_cost, &s2e_cost, &e2s_cost, 20));
}
