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
    vertex: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    component: &mut Vec<usize>,
) {
    visited[vertex] = true;
    component.push(vertex);

    for &child in &graph[vertex] {
        if !visited[child] {
            dfs(child, graph, visited, component);
        }
    }
}
fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: usize = sc.next();
    let e: usize = sc.next();

    let mut graph = vec![vec![]; n + 1];
    let mut cc = Vec::new();
    let mut visited = vec![false; n + 1];

    for _ in 1..=e {
        let x: usize = sc.next();
        let y: usize = sc.next();

        graph[x].push(y);
        graph[y].push(x);
    }

    for i in 1..=n {
        if !visited[i] {
            let mut component = Vec::new();
            dfs(i, &graph, &mut visited, &mut component);
            cc.push(component);
        }
    }

    writeln!(out, "{}", cc.len()).unwrap();
    writeln!(out, "{:?}", cc).unwrap()
}
