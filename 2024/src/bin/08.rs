//  rustc 08.rs && ./08

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "8.in";

fn mark_antinodes(
    is_antinode: &mut Vec<Vec<bool>>,
    (x0, y0): (i32, i32),
    (dx, dy): (i32, i32),
    infinity: bool,
) {
    let n_row32 = is_antinode.len() as i32;
    let n_col32 = is_antinode[0].len() as i32;
    let (mut x, mut y) = (x0, y0);
    loop {
        x += dx;
        y += dy;
        if x < 0 || y < 0 || x >= n_row32 || y >= n_col32 {
            break;
        }
        is_antinode[x as usize][y as usize] = true;
        if !infinity {
            break;
        }
    }
}

fn find_antinodes(
    is_antinode: &mut Vec<Vec<bool>>,
    (x0, y0): (usize, usize),
    (x1, y1): (usize, usize),
    infinity: bool,
) {
    let (xa, ya) = (x0 as i32, y0 as i32);
    let (xb, yb) = (x1 as i32, y1 as i32);
    let dx = (xa - xb).abs();
    let dy = (ya - yb).abs();
    let (dxa, mut dya) = (-dx, dy);
    let (dxb, mut dyb) = (dx, dy);
    if y0 > y1 {
        // case "/" or "-"
        dyb *= -1;
    } else {
        // case "\" or "|"
        dya *= -1;
    }
    if infinity {
        is_antinode[x0][y0] = true;
        is_antinode[x1][y1] = true;
    }
    mark_antinodes(is_antinode, (xa, ya), (dxa, dya), infinity);
    mark_antinodes(is_antinode, (xb, yb), (dxb, dyb), infinity);
}

fn solve(map: &Vec<Vec<char>>, infinity: bool) -> i32 {
    let (n_row, n_col) = (map.len(), map[0].len());
    let mut is_antinode: Vec<Vec<bool>> = vec![vec![false; n_col]; n_row];
    let mut cells: Vec<(usize, usize)> = vec![];
    for x in 0..n_row {
        for y in 0..n_col {
            if map[x][y] != '.' {
                cells.push((x, y));
            }
        }
    }
    cells.sort_by(|a, b| map[a.0][a.1].cmp(&map[b.0][b.1]));
    let n_cells = cells.len();
    for i in 0..n_cells {
        let (x0, y0) = cells[i];
        for k in i + 1..n_cells {
            let (x1, y1) = cells[k];
            assert!(x0 != x1 || y0 != y1);
            if map[x0][y0] != map[x1][y1] {
                break;
            }
            if x0 < x1 {
                find_antinodes(&mut is_antinode, (x0, y0), (x1, y1), infinity);
            } else {
                find_antinodes(&mut is_antinode, (x1, y1), (x0, y0), infinity);
            }
        }
    }
    return is_antinode
        .iter()
        .flat_map(|v| v.iter())
        .map(|x| *x as i32)
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

    println!("{}", solve(&map, false));
    println!("{}", solve(&map, true));
}
