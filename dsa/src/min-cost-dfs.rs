#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    vertex: usize,
    weight: usize,
}

fn dfs(
    current: usize,
    target: usize,
    graph: &Vec<Vec<Pair>>,
    visited: &mut Vec<bool>,
    cost: usize,
    min_cost: &mut usize,
) {
    if current == target {
        *min_cost = (*min_cost).min(cost);
        return;
    }

    visited[current] = true;

    for &child in &graph[current] {
        if !visited[child.vertex] {
            dfs(child.vertex, target, graph, visited, cost + child.weight, min_cost);
        }
    }

    visited[current] = false; // Allow re-visiting for different paths
}

fn main() {
    unimplemented!()
}

