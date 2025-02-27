// // Define the Graph structure
// struct Graph {
//     vertices: Vec<String>,
//     edges: Vec<(String, String, u32)>,
// }

// impl Graph {
//     // Create a new graph with the given vertices and edges
//     fn new(vertices: Vec<String>, edges: Vec<(String, String, u32)>) -> Self {
//         Graph { vertices, edges }
//     }

//     // Print the minimum spanning tree
//     fn prim_traversal(&self) {
//         let mut visited = vec![false; self.vertices.len()];
//         let mut mst_edges = Vec::new();
//         let mut min_weight = 0;

//         for vertex in &self.vertices {
//             if !visited[vertex.len() - 1] {
//                 prim_mst_vertex(vertex, &mut visited, &mut mst_edges, &self.edges);
//                 min_weight += self.edges[0].2;
//             }
//         }

//         println!("Minimum Spanning Tree:");
//         for edge in &mst_edges {
//             println!("{} -> {} : {}", edge.0, edge.1, edge.2);
//         }
//     }
// }

// // Recursive function to find the minimum spanning tree
// fn prim_mst_vertex(
//     current_vertex: &str,
//     visited: &mut [bool],
//     mst_edges: &mut Vec<(String, String, u32)>,
//     edges: &[(&str, &str, u32)],
// ) {
//     for edge in edges {
//         if *edge.0 != current_vertex && !visited[edge.0.len() - 1] {
//             visited[edge.0.len() - 1] = true;
//             mst_edges.push(edge.clone());
//             println!("{} -> {} : {}", edge.0, edge.1, edge.2);
//             prim_mst_vertex(&edge.1, visited, mst_edges, edges);
//         }
//     }
// }

// fn main() {
//     // Create a graph with 5 vertices and edges
//     let vertices = vec![
//         "A".to_string(),
//         "B".to_string(),
//         "C".to_string(),
//         "D".to_string(),
//         "E".to_string(),
//     ];
//     let edges = vec![
//         ("A", "B", 2),
//         ("B", "C", 3),
//         ("A", "C", 4),
//         ("B", "D", 1),
//         ("C", "D", 5),
//         ("A", "E", 6),
//         ("B", "E", 7),
//         ("C", "E", 8),
//     ];

//     // let graph = Graph::new(vertices, edges);
//     // graph.prim_traversal();
// }
fn main() {
    println!("HELLO")
}
