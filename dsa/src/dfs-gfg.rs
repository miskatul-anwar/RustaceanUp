fn dfs(vertex: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, trip: &mut Vec<usize>) {
    visited[vertex] = true;

    trip.push(vertex);

    for &child in &graph[vertex] {
        if !visited[child] {
            dfs(child, graph, visited, trip);
        }
    }
}

fn main() {
    // let graph = vec![vec![2, 3, 1], vec![0], vec![0, 4], vec![0], vec![2]];
    let graph = vec![
        vec![1, 2],
        vec![0, 2],
        vec![0, 1, 2, 3, 4],
        vec![2],
        vec![2],
    ];

    let mut visited = vec![false; 5];
    let mut trip = Vec::new();

    dfs(3, &graph, &mut visited, &mut trip);

    for i in 0..trip.len() {
        if i == 4 {
            println!("{}", trip[i])
        } else {
            print!("{} -> ", trip[i])
        }
    }
}
