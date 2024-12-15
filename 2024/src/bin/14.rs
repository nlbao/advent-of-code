//  cargo run --bin 14
//
// Status:  passed puzzle 1.

use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;
use std::time;

const FILE_PATH: &str = "14.in";

const N_ROW: usize = 103;
const N_COL: usize = 101;
// const N_ROW: usize = 7;
// const N_COL: usize = 11;

const N_ROW_32: i32 = N_ROW as i32;
const N_COL_32: i32 = N_COL as i32;

// const LEFT: usize = 0;
// const RIGHT: usize = 1;
// const TOP: usize = 2;
// const BOTTOM: usize = 3;

// fn get_adj_cells(x: usize, y: usize, n_row: usize, n_col: usize) -> Vec<(usize, usize, usize)> {
//     let dx: Vec<i32> = vec![0, 0, -1, 1, -1, -1, 1, 1];
//     let dy: Vec<i32> = vec![-1, 1, 0, 0, -1, 1, -1, 1];
//     let (n_row32, n_col32) = (n_row as i32, n_col as i32);
//     let (x_32, y_32) = (x as i32, y as i32);
//     let mut ans: Vec<(usize, usize, usize)> = vec![];
//     for k in 0..dx.len() {
//         let i = x_32 + dx[k];
//         let j = y_32 + dy[k];
//         if i < 0 || j < 0 || i >= n_row32 || j >= n_col32 {
//             continue;
//         }
//         ans.push((k, i as usize, j as usize));
//     }
//     return ans;
// }

fn move_robot((x0, y0): &(usize, usize), (vx, vy): &(i32, i32), loops: i32) -> (usize, usize) {
    let (x, y) = (x0.clone() as i32, y0.clone() as i32);
    return (
        (((x + vx * loops) % N_ROW_32 + N_ROW_32) % N_ROW_32) as usize,
        (((y + vy * loops) % N_COL_32 + N_COL_32) % N_COL_32) as usize,
    );
    // let (mut x, mut y) = (x0.clone() as i32, y0.clone() as i32);
    // for _ in 0..loops {
    //     x = ((x + vx) % N_ROW_32 + N_ROW_32) % N_ROW_32;
    //     y = ((y + vy) % N_COL_32 + N_COL_32) % N_COL_32;
    // }
    // return (x as usize, y as usize);
}

fn solve(
    pos: &Vec<(usize, usize)>,
    velo: &Vec<(i32, i32)>,
    loops: i32,
) -> (i64, i64, i64, i64, Vec<Vec<i32>>) {
    let n = pos.len();
    let half_n_row = N_ROW >> 1;
    let half_n_col = N_COL >> 1;
    let mut map: Vec<Vec<i32>> = vec![vec![0; N_COL]; N_ROW];
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for i in 0..n {
        assert!(pos[i].0 < N_ROW && pos[i].1 < N_COL);
        let (x, y) = move_robot(&pos[i], &velo[i], loops);
        // println!("{}     {} {} : {} {}", i, pos[i].0, pos[i].1, x, y);
        map[x][y] += 1;
        if x == half_n_row || y == half_n_col {
            continue;
        }
        if x < half_n_row {
            // let t = &q1;
            // *t += 1;
            if y < half_n_col {
                q1 += 1;
            } else {
                q2 += 1;
            }
        } else {
            if y < half_n_col {
                q3 += 1;
            } else {
                q4 += 1;
            }
        }
    }
    // println!("\n{} {} {} {}", q1, q2, q3, q4);
    return (q1, q2, q3, q4, map);
}

fn is_symmetric_vertically(map: &Vec<Vec<i32>>, ok_percent: i32) -> bool {
    // let total = N_ROW_32 * N_COL_32;
    let mut total = 0;
    let mut cnt_ok = 0;
    for x in 0..N_ROW {
        for y in 0..N_COL {
            total += (map[x][y] > 0) as i32;
            // let zero = map[x][N_COL - 1 - y] == 0;
            // if ((map[x][y] == 0) ^ zero) == false {
            if (map[x][y] == 0) || (map[x][N_COL - 1 - y] == 0) {
                // return false;
                // cnt_ok -= 1;
            } else {
                cnt_ok += 1;
            }
        }
    }
    // println!(
    //     "cnt_ok = {}, total = {},    percent = {}",
    //     cnt_ok,
    //     total,
    //     (100 * cnt_ok / total)
    // );
    return (100 * cnt_ok / total) >= ok_percent;
    // return true;
}

fn print_map(map: &Vec<Vec<i32>>) {
    for x in 0..N_ROW {
        for y in 0..N_COL {
            if map[x][y] == 0 {
                print!(".");
            } else {
                print!("{}", map[x][y]);
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut pos: Vec<(usize, usize)> = vec![];
    let mut velo: Vec<(i32, i32)> = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        for (_, [y, x, vy, vx]) in re.captures_iter(&l).map(|c| c.extract()) {
            pos.push((x.parse().unwrap(), y.parse().unwrap()));
            velo.push((vx.parse().unwrap(), vy.parse().unwrap()));
        }
    }

    // let (q1, q2, q3, q4, _) = solve(&pos, &velo, 100);
    // println!("{}", q1 * q2 * q3 * q4);

    let sleep_time = time::Duration::from_millis(100);
    // 1601348
    // 114
    // let mut i = 113;
    // let mut i = 20000000;
    let mut i = 0;
    loop {
        i += 1;
        let (q1, q2, q3, q4, map) = solve(&pos, &velo, i);

        if i % 1000 == 0 {
            println!("loops = {}", i);
        }

        // print_map(&map);
        // println!("loops = {}", i);
        // thread::sleep(sleep_time);
        // break;

        // 703
        let ok_percent = 100;
        if (100 * min(q1, q2) / max(q1, q2)) < ok_percent {
            continue;
        }
        if (100 * min(q3, q4) / max(q3, q4)) < ok_percent {
            continue;
        }
        print_map(&map);
        println!("loops = {}", i);
        break;

        // // 6680000
        // // if q1 == q2 && q3 == q4 && is_symmetric_vertically(&map) {
        // if is_symmetric_vertically(&map, 18) {
        //     print_map(&map);
        //     println!("loops = {}", i);
        //     break;
        // }
    }
}
