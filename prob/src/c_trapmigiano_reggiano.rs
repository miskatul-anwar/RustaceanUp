#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn dfs(node: usize, parent: usize, tree: &Vec<Vec<usize>>, dist: &mut Vec<i32>) {
    for &neighbor in &tree[node] {
        if neighbor != parent {
            dist[neighbor] = dist[node] + 1;
            dfs(neighbor, node, tree, dist);
        }
    }
}

fn solve() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let node_count: usize = sc.next();
    let start: usize = sc.next();
    let end: usize = sc.next();

    let mut tree = vec![vec![]; node_count + 1];
    for _ in 0..node_count - 1 {
        let u: usize = sc.next();
        let v: usize = sc.next();
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut dist = vec![-1; node_count + 1];
    dist[end] = 0;
    dfs(end, 0, &tree, &mut dist);

    let mut perm: Vec<usize> = (1..=node_count).collect();
    perm.sort_by(|&a, &b| dist[b].cmp(&dist[a]));

    if dist[start] == -1 {
        writeln!(out, "-1").unwrap();
    } else {
        for p in perm {
            write!(out, "{} ", p).unwrap();
        }
        writeln!(out).unwrap();
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        solve();
    }
}
