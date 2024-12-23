//  rustc 23.rs && ./23

use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "23.in";

fn part1(names: &Vec<String>, adj: &Vec<Vec<usize>>) -> i64 {
    let mut ans = 0;
    let n = names.len();
    for u in 0..n {
        for i in 0..adj[u].len() {
            let v = adj[u][i];
            if v <= u {
                continue;
            }
            for k in 0..adj[v].len() {
                let t = adj[v][k];
                if t <= v || !adj[t].contains(&u) {
                    continue;
                }
                if names[u].starts_with("t")
                    || names[v].starts_with("t")
                    || names[t].starts_with("t")
                {
                    ans += 1;
                }
            }
        }
    }
    return ans;
}

fn is_clique(candidates: &Vec<usize>, connected: &Vec<Vec<bool>>) -> bool {
    for v in candidates {
        for t in candidates {
            if !connected[*v][*t] {
                return false;
            }
        }
    }
    return true;
}

fn part2(names: &Vec<String>, adj: &Vec<Vec<usize>>) -> String {
    let n = names.len();
    let mut connected: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for u in 0..n {
        connected[u][u] = true;
        for v in adj[u].iter() {
            connected[u][*v] = true;
        }
    }
    let mut max_clique = vec![];
    for u in 0..n {
        println!("u = {}, clique = {:?}", u, max_clique);
        let n_adj = adj[u].len();
        for mask in 0..(1 << n_adj) {
            let candidates: Vec<usize> = (0..n_adj)
                .filter(|i| (mask >> i) & 1 > 0)
                .map(|i| adj[u][i])
                .chain(vec![u].into_iter())
                .collect();
            if candidates.len() > max_clique.len() && is_clique(&candidates, &connected) {
                max_clique = candidates;
            }
        }
    }
    let mut clique_names: Vec<String> = max_clique
        .into_iter()
        .map(|u| names[u].to_string())
        .collect();
    clique_names.sort();
    return clique_names.join(",");
}

fn main() {
    let mut n_node = 0;
    let mut ids = HashMap::new();
    let mut names = vec![];
    let mut adj = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        let uv: Vec<String> = l.split("-").map(|x| x.to_string()).collect();
        let (mut u, mut v) = (0, 0);
        for i in 0..2 {
            let id = *ids.entry(uv[i].to_owned()).or_insert(n_node);
            if id == n_node {
                n_node += 1;
                names.push(uv[i].to_owned());
                adj.push(vec![]);
            }
            if i == 0 {
                u = id;
            } else {
                v = id;
            }
        }
        assert!(u != v);
        adj[u].push(v);
        adj[v].push(u);
    }

    for u in 0..n_node {
        adj[u].sort();
        adj[u].dedup();
    }

    let max_degree = (0..n_node).into_iter().map(|i| adj[i].len()).max().unwrap();
    println!("n_node = {}", n_node);
    println!("max_degree = {}", max_degree);

    println!("ans_part1 = {}", part1(&names, &adj));
    println!("ans_part2 = {}", part2(&names, &adj));
}
