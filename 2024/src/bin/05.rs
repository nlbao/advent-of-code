//  rustc 05.rs && ./05

use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "5.in";

fn topological_sort(adj: &Vec<Vec<usize>>, allowed_nodes: &Vec<usize>) -> Vec<i32> {
    let n = adj.len();
    let mut is_allowed: Vec<bool> = vec![false; n];
    let mut degrees: Vec<i32> = vec![-1; n];
    for u in allowed_nodes {
        is_allowed[*u] = true;
        degrees[*u] = 0;
    }
    for u in allowed_nodes {
        for i in 0..adj[*u].len() {
            let v = adj[*u][i];
            if is_allowed[v] {
                degrees[v] += 1;
            }
        }
    }
    let mut levels: Vec<i32> = vec![-1; n];
    let mut q: VecDeque<usize> = VecDeque::new();
    for u in 0..n {
        if degrees[u] == 0 {
            q.push_back(u);
            levels[u] = 0;
        }
    }
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for i in 0..adj[u].len() {
            let v = adj[u][i];
            levels[v] = max(levels[v], levels[u] + 1);
            degrees[v] -= 1;
            if degrees[v] == 0 {
                q.push_back(v);
            }
        }
    }
    return levels;
}

fn is_valid_order(update: &Vec<usize>, levels: &Vec<i32>) -> bool {
    let mut pre_level = -1;
    for u in update {
        let l = levels[*u];
        if l < pre_level {
            return false;
        }
        pre_level = l;
    }
    return true;
}

fn part1(adj: &Vec<Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut ans = 0;
    for update in updates {
        let levels = topological_sort(adj, update);
        if is_valid_order(update, &levels) {
            ans += update[update.len() >> 1];
        }
    }
    return ans;
}

fn part2(adj: &Vec<Vec<usize>>, updates: &Vec<Vec<usize>>) -> usize {
    let mut ans = 0;
    for update in updates {
        let levels = topological_sort(adj, update);
        if is_valid_order(update, &levels) {
            continue;
        }
        let mut new_update = update.clone();
        new_update.sort_by(|u, v| levels[*u].cmp(&levels[*v]));
        ans += new_update[new_update.len() >> 1];
    }
    return ans;
}

fn main() {
    let mut n: usize = 0;
    let mut edges: Vec<(usize, usize)> = vec![];
    let mut updates: Vec<Vec<usize>> = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut reading_orders = true;
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.len() == 0 {
            reading_orders = false;
            continue;
        }
        if reading_orders {
            let pair: Vec<usize> = l.split("|").map(|s| s.parse().unwrap()).collect();
            let (u, v) = (pair[0], pair[1]);
            n = max(n, max(u, v) + 1);
            edges.push((u, v));
        } else {
            let update: Vec<usize> = l.split(",").map(|s| s.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for (u, v) in edges {
        assert!(u != v);
        adj[u].push(v);
    }

    println!("{}", part1(&adj, &updates));
    println!("{}", part2(&adj, &updates));
}
