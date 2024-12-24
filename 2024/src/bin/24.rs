//  rustc 24.rs && ./24

use std::collections::HashMap;
use std::fs::File;
use std::i64;
use std::io::prelude::*;
use std::io::BufReader;

const FILE_PATH: &str = "24.in";

enum Op {
    AND,
    OR,
    XOR,
}

fn get_node_id(name: &str, ids: &mut HashMap<String, usize>, names: &mut Vec<String>) -> usize {
    let n_node = names.len();
    let id = *ids.entry(name.to_owned()).or_insert(n_node);
    if id == n_node {
        names.push(name.to_owned());
    }
    return id;
}

fn calc(f0: &i8, f1: &i8, op: &Op) -> i8 {
    return match op {
        Op::AND => f0 & f1,
        Op::OR => f0 | f1,
        Op::XOR => f0 ^ f1,
    };
}

fn part1(
    names: &Vec<String>,
    ids: &HashMap<String, usize>,
    inits: &Vec<i8>,
    edges: &Vec<(usize, usize, Op, usize)>,
) -> i64 {
    let n = ids.len();
    let mut f = inits.clone();
    for _ in 0..n {
        let mut stop = true;
        for (u0, u1, op, v) in edges {
            let (f0, f1, fv) = (f[*u0], f[*u1], f[*v]);
            let new_f = calc(&f0, &f1, op);
            if f0 < 0 || f1 < 0 || fv == new_f {
                continue;
            }
            assert!(fv == -1);
            f[*v] = new_f;
            stop = false;
            println!("val( {} ) = {}", names[*v], new_f);
        }
        if stop {
            break;
        }
    }
    //
    // let mut a: Vec<&i8> = ids
    //     .iter()
    //     .filter(|(name, _)| name.starts_with('z'))
    //     .map(|(_, u)| &f[*u])
    //     .collect();
    // a.sort();
    // return (0..a.len()).into_iter().map(|i| (*a[i] as i64) << i).sum();
    let mut a: Vec<String> = names
        .iter()
        .filter(|x| x.starts_with('z'))
        .map(|x| x.to_string())
        .collect();
    a.sort();
    return (0..a.len())
        .into_iter()
        .map(|i| (f[*ids.get(&a[i]).unwrap()] as i64) << i)
        .sum();
}

fn main() {
    let mut ids = HashMap::new();
    let mut names = vec![];
    // let mut adj = vec![];
    let mut edges = vec![];
    let mut inits: Vec<i8> = vec![];

    let file = File::open(FILE_PATH).expect("Error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let l = line.unwrap().to_owned();
        if l.is_empty() {
            continue;
        } else if l.contains(":") {
            let t: Vec<&str> = l.split(": ").collect();
            let u = get_node_id(t[0], &mut ids, &mut names);
            inits.resize(names.len(), -1);
            inits[u] = t[1].parse().unwrap();
        } else {
            let t: Vec<&str> = l.split(" -> ").collect();
            let v = get_node_id(t[1], &mut ids, &mut names);
            let tu: Vec<&str> = t[0].split(" ").collect();
            let u0 = get_node_id(tu[0], &mut ids, &mut names);
            let u1 = get_node_id(tu[2], &mut ids, &mut names);
            // println!("op = {}", tu[1]);
            let op = match tu[1] {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => unreachable!(),
            };
            edges.push((u0, u1, op, v));
            inits.resize(names.len(), -1);
        }
        // adj[u].push(v);
        // adj[v].push(u);
    }

    // for u in 0..n_node {
    //     adj[u].sort();
    //     adj[u].dedup();
    // }

    // let max_degree = (0..n_node).into_iter().map(|i| adj[i].len()).max().unwrap();
    println!("n_node = {}", names.len());
    // println!("max_degree = {}", max_degree);

    println!("ans_part1 = {}", part1(&names, &ids, &inits, &edges));
    // println!("ans_part2 = {}", part2(&names, &adj));
}
