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
fn dfs(vertex: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[vertex] = true;

    for &child in &graph[vertex] {
        if !visited[child] {
            dfs(child, graph, visited);
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: usize = sc.next();
    let e: usize = sc.next();

    let mut graph = vec![vec![]; n + 1];
    let mut visited = vec![false; n + 1];

    for i in 1..=e {
        let x: usize = sc.next();
        let y: usize = sc.next();

        graph[x].push(y); /* Undirected Graph!! */
        graph[y].push(x);
    }

    let mut count = 0;

    for i in 1..=n {
        if visited[i] {
            continue;
        }

        dfs(i, &graph, &mut visited);
        count += 1
    }

    writeln!(out, "{}", count).unwrap()
}
