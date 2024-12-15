//  rustc 15.rs && ./15

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "15.in";
const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;

const DX: &'static [i32] = &[0, 0, -1, 1];
const DY: &'static [i32] = &[-1, 1, 0, 0];

fn move2dir(ch: char) -> usize {
    return match ch {
        '^' => TOP,
        'v' => BOTTOM,
        '>' => RIGHT,
        '<' => LEFT,
        _ => unreachable!(),
    };
}

fn print_map(map: &Vec<Vec<char>>) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            print!("{}", map[x][y]);
        }
        println!();
    }
    println!();
}

fn find_robot_pos(map: &Vec<Vec<char>>) -> (usize, usize) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == '@' {
                return (x, y);
            }
        }
    }
    unreachable!();
}

fn sum_gps(map: &Vec<Vec<char>>) -> i64 {
    let mut ans = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 'O' || map[x][y] == '[' {
                ans += (100 * x + y) as i64;
            }
        }
    }
    return ans;
}

fn try_move_left_or_right(map: &mut Vec<Vec<char>>, rx: usize, ry: usize, dir: usize) -> bool {
    let n_row32 = map.len() as i32;
    let n_col32 = map[0].len() as i32;
    let i = rx as i32 + DX[dir];
    let j = ry as i32 + DY[dir];
    if i < 0 || j < 0 || i >= n_row32 || j >= n_col32 {
        return false;
    }
    let (x, y) = (i as usize, j as usize);
    if map[x][y] == '#' {
        return false;
    }
    if map[x][y] == 'O' || map[x][y] == '[' || map[x][y] == ']' {
        if !try_move_left_or_right(map, x, y, dir) {
            return false;
        }
    }
    map[x][y] = map[rx][ry]; // move (rx, ry) -> (x, y)
    map[rx][ry] = '.';
    return true;
}

fn try_move_top_or_bottom(
    map: &mut Vec<Vec<char>>,
    rx: usize,
    ry_set: &HashSet<usize>,
    dir: usize,
) -> bool {
    let n_row = map.len();
    if (dir == TOP && rx == 0) || (dir == BOTTOM && rx == n_row - 1) {
        return false;
    }
    let x = (rx as i32 + DX[dir]) as usize;
    let mut next_row_y_set: HashSet<usize> = HashSet::new();
    for y_ in ry_set {
        let y = *y_;
        let c = map[x][y];
        if c == '#' {
            return false;
        }
        if c != '.' {
            next_row_y_set.insert(y);
            let _ = match c {
                ']' => next_row_y_set.insert(y - 1),
                '[' => next_row_y_set.insert(y + 1),
                _ => true,
            };
        }
    }
    if !next_row_y_set.is_empty() {
        if !try_move_top_or_bottom(map, x, &next_row_y_set, dir) {
            return false;
        }
    }

    // move
    for y_ in ry_set {
        let y = *y_;
        map[x][y] = map[rx][y]; // move (rx, y) -> (x, y)
        map[rx][y] = '.';
    }
    return true;
}

fn solve(map: &mut Vec<Vec<char>>, moves: &Vec<char>) -> i64 {
    let (mut rx, mut ry) = find_robot_pos(&map);
    for i in 0..moves.len() {
        let dir = move2dir(moves[i]);
        let can_move = if dir == LEFT || dir == RIGHT {
            try_move_left_or_right(map, rx, ry, dir)
        } else {
            try_move_top_or_bottom(map, rx, &HashSet::from([ry]), dir)
        };
        if can_move {
            rx = (rx as i32 + DX[dir]) as usize;
            ry = (ry as i32 + DY[dir]) as usize;
        }

        // println!("move {}-th : {}", i, moves[i]);
        // print_map(map);
        // let mut input = String::new();
        // io::stdin().read_line(&mut input).expect("error");
    }
    return sum_gps(map);
}

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut map_2: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<char> = vec![];
    let mut is_map = true;

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.is_empty() {
            is_map = false;
            continue;
        }
        if is_map {
            map.push(l.chars().collect());
            let v2: Vec<char> = l
                .chars()
                .map(|c| match c {
                    '#' => "##",
                    'O' => "[]",
                    '.' => "..",
                    '@' => "@.",
                    _ => unreachable!(),
                })
                .flat_map(|s| s.chars())
                .collect();
            map_2.push(v2);
        } else {
            l.chars().for_each(|c| moves.push(c));
        }
    }

    print_map(&map_2);
    println!("{}", solve(&mut map, &moves));
    println!("{}", solve(&mut map_2, &moves));
}
