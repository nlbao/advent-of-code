//  rustc 16.rs && ./16

use std::collections::BTreeSet;
use std::fs::File;
use std::i64;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "16.in";
const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;

const OPPOSITE_DIRS: &'static [usize] = &[RIGHT, LEFT, BOTTOM, TOP];
const ROTATED_DIRS: &'static [[usize; 2]] =
    &[[TOP, BOTTOM], [TOP, BOTTOM], [LEFT, RIGHT], [LEFT, RIGHT]];

const DX: &'static [i32] = &[0, 0, -1, 1];
const DY: &'static [i32] = &[-1, 1, 0, 0];

fn shortest_path(
    map: &Vec<Vec<char>>,
    (sx, sy, sdirs): (usize, usize, &Vec<usize>),
    reverse: bool,
) -> Vec<Vec<Vec<i64>>> {
    let (n_row, n_col) = (map.len(), map[0].len());
    let (n_row_32, n_col_32) = (n_row as i32, n_col as i32);
    let mut cost: Vec<Vec<Vec<i64>>> = vec![vec![vec![i64::MAX; 4]; n_col]; n_row];
    let mut heap = BTreeSet::new();
    for sdir in sdirs {
        cost[sx][sy][*sdir] = 0;
        heap.insert((0, (sx, sy), *sdir));
    }
    while !heap.is_empty() {
        let (c, (x, y), dir) = heap.pop_first().unwrap();
        assert!(c == cost[x][y][dir]);
        let mut next_cells: Vec<(usize, usize, usize)> = vec![];
        let opposite_dir = OPPOSITE_DIRS[dir];
        // move forward
        {
            let new_dir = if reverse { opposite_dir } else { dir };
            let i = (x as i32) + DX[new_dir];
            let j = (y as i32) + DY[new_dir];
            if i >= 0 && j >= 0 && i < n_row_32 && j < n_col_32 {
                next_cells.push((i as usize, j as usize, dir)); // not new_dir in case of reversal
            }
        }
        // rotate
        for new_dir in ROTATED_DIRS[dir] {
            next_cells.push((x, y, new_dir));
        }
        // update next_cells' cost
        for (xx, yy, new_dir) in next_cells {
            if map[xx][yy] == '#' {
                continue;
            }
            let new_cost = c + (if new_dir == dir { 1 } else { 1000 });
            if new_cost < cost[xx][yy][new_dir] {
                heap.remove(&(cost[xx][yy][new_dir], (xx, yy), new_dir));
                cost[xx][yy][new_dir] = new_cost;
                heap.insert((new_cost, (xx, yy), new_dir));
            }
        }
    }
    return cost;
}

fn part1(cost: &Vec<Vec<Vec<i64>>>, (ex, ey): (usize, usize)) -> i64 {
    return (0..4).into_iter().map(|i| cost[ex][ey][i]).min().unwrap();
}

fn part2(s2e_cost: &Vec<Vec<Vec<i64>>>, e2s_cost: &Vec<Vec<Vec<i64>>>, target_cost: i64) -> usize {
    let (n_row, n_col) = (s2e_cost.len(), s2e_cost[0].len());
    let mut ans = 0;
    for x in 0..n_row {
        for y in 0..n_col {
            ans += (0..4).into_iter().any(|dir| {
                if s2e_cost[x][y][dir] == i64::MAX || e2s_cost[x][y][dir] == i64::MAX {
                    false
                } else {
                    s2e_cost[x][y][dir] + e2s_cost[x][y][dir] == target_cost
                }
            }) as usize;
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

    let sdirs = vec![RIGHT];
    let edirs = vec![LEFT, RIGHT, TOP, BOTTOM];
    let s2e_cost = shortest_path(&map, (sx, sy, &sdirs), false);
    let e2s_cost = shortest_path(&map, (ex, ey, &edirs), true);
    let target_cost = part1(&s2e_cost, (ex, ey));
    println!("ans_part1 = {}", target_cost);
    println!("ans_part2 = {}", part2(&s2e_cost, &e2s_cost, target_cost));
}
