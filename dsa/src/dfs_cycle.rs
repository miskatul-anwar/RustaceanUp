use std::io::{stdout, BufWriter, Write};

fn dfs(vertex: usize, parent: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> bool {
    visited[vertex] = true;

    for &child in &graph[vertex] {
        if !visited[child] {
            if dfs(child, vertex, graph, visited) {
                return true;
            }
        } else if child != parent {
            return true;
        }
    }

    false
}

fn main() {
    let out = &mut BufWriter::new(stdout());

    let mut graph = vec![vec![]; 3];
    let mut visited = vec![false; 3];

    graph[1].push(0);
    graph[0].push(1);
    graph[0].push(2);
    graph[2].push(0);
    graph[1].push(2);
    graph[2].push(1);

    let mut has_cycle = false;

    for i in 0..3 {
        if !visited[i] && dfs(i, 0, &graph, &mut visited) {
            has_cycle = true;
            break;
        }
    }

    if has_cycle {
        writeln!(out, "Cycle detected").unwrap();
    } else {
        writeln!(out, "No cycle detected").unwrap();
    }
}
