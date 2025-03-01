fn dfs(node: usize, parent: usize, graph: &Vec<Vec<usize>>) {
    println!("Visiting node: {}", node);

    for &child in &graph[node] {
        if child == parent {
            continue; // Prevents revisiting the parent in an undirected tree
        }
        dfs(child, node, graph);
    }
}

fn main() {
    // Define the number of nodes in the tree
    let n = 7; // Example tree with 7 nodes (0 to 6)

    // Adjacency list representation of the tree
    let mut graph = vec![vec![]; n];

    // Define edges (assuming 0-based indexing)
    let edges = vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)];

    // Build the adjacency list
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u); // Since it's an undirected tree
    }

    println!("Starting DFS traversal from node 0:\n");
    dfs(0, usize::MAX, &graph); // Start DFS from the root (0), with no parent
}
