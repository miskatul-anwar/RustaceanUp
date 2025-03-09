use std::io::{stdin, stdout, BufWriter};

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

fn dfs(
    vertex: usize,
    graph: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    even_nodes: &mut Vec<usize>,
) {
    visited[vertex] = true;

    if vertex % 2 == 0 {
        even_nodes.push(vertex)
    }

    print!("{} -> ", vertex);
    for &child in &graph[vertex] {
        if !visited[child] {
            dfs(child, graph, visited, even_nodes);
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let _out = &mut BufWriter::new(stdout());

    let n: usize = sc.next();
    let m: usize = sc.next();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n + 1];
    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut even_nodes: Vec<usize> = Vec::new();

    for _ in 0..m {
        let v1: usize = sc.next();
        let v2: usize = sc.next();

        graph[v1].push(v2);
        graph[v2].push(v1);
    }

    dfs(1, &graph, &mut visited, &mut even_nodes);

    println!("Even Nodes: {:?}", even_nodes);
}
