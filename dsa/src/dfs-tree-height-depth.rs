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

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/
fn dfs(
    node: usize,
    parent: usize,
    graph: &Vec<Vec<usize>>,
    height: &mut Vec<usize>,
    depth: &mut Vec<usize>,
) {
    for &child in &graph[node] {
        if child == parent {
            continue;
        }

        depth[child] = depth[node] + 1;
        dfs(child, node, graph, height, depth);
        height[node] = height[node].max(height[child] + 1);
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {}
}
