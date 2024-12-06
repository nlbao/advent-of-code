//  rustc 06.rs && ./06

use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "6.in";

fn get_direction(x: char) -> usize {
    return match x {
        '>' => 0,
        'V' => 1,
        '<' => 2,
        '^' => 3,
        _ => 999999,
    };
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize, usize) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let direction = get_direction(map[x][y]);
            if direction < 4 {
                return (x, y, direction);
            }
        }
    }
    unreachable!();
}

fn part1(map: &Vec<Vec<char>>) -> i32 {
    let dx: Vec<i32> = vec![0, 1, 0, -1]; // >, V, <, ^
    let dy: Vec<i32> = vec![1, 0, -1, 0];
    let (n_row, n_col) = (map.len(), map[0].len());
    let (n_row32, n_col32) = (n_row as i32, n_col as i32);

    let mut ans: i32 = 0;
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; n_col]; n_row];
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_back(find_start(map));
    while !q.is_empty() {
        let (x, y, direction) = q.pop_front().unwrap();
        if visited[x][y][direction] {
            return -1; // it's a loop
        }
        visited[x][y][direction] = true;
        let count: i32 = [0, 1, 2, 3].map(|d| visited[x][y][d] as i32).iter().sum();
        if count == 1 {
            ans += 1; // visit (x, y) the first time.
        }
        let (i, j) = (x as i32 + dx[direction], y as i32 + dy[direction]);
        if i < 0 || j < 0 || i >= n_row32 || j >= n_col32 {
            break; // out of the map.
        }
        let (xx, yy) = (i as usize, j as usize);
        if map[xx][yy] != '#' {
            q.push_back((xx, yy, direction));
        } else {
            q.push_back((x, y, (direction + 1) % 4));
        }
    }
    return ans;
}

fn part2(map: &Vec<Vec<char>>) -> i32 {
    let (n_row, n_col) = (map.len(), map[0].len());
    let (start_x, start_y, _) = find_start(map);
    let mut ans: i32 = 0;
    let mut new_map = map.clone();
    for x in 0..n_row {
        for y in 0..n_col {
            if x == start_x && y == start_y || map[x][y] != '.' {
                continue;
            }
            new_map[x][y] = '#'; // try to place an obstacle
            if part1(&new_map) == -1 {
                ans += 1; // a loop.
            }
            new_map[x][y] = '.'; // reset
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

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}
