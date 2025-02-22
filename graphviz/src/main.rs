//! This is the best way to draw graphs in rust!!!

use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn generate_graph() {
    let mut graph = DiGraph::new();

    let aa = graph.add_node("aa");
    let be = graph.add_node("be");
    let d = graph.add_node("d");
    let aaa = graph.add_node("aaa");
    let bbb = graph.add_node("bbb");
    let a2 = graph.add_node("a2");
    let b2 = graph.add_node("b2");

    graph.add_edge(aa, be, "");
    graph.add_edge(be, d, "");
    graph.add_edge(d, aaa, "");
    graph.add_edge(aa, aaa, "");
    graph.add_edge(aaa, bbb, "");
    graph.add_edge(a2, b2, "");

    let dot_format = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let mut file = File::create("bin/output_graph.dot").expect("Failed to create file");
    file.write_all(dot_format.as_bytes())
        .expect("Failed to write to file");
    println!("Graph saved as output_graph.dot");

    generate_svg()
}

fn generate_svg() {
    let output = Command::new("dot")
        .args([
            "-Tsvg",
            "bin/output_graph.dot",
            "-o",
            "assets/output_graph.svg",
        ])
        .output()
        .expect("Failed to execute Graphviz");

    if output.status.success() {
        println!("SVG generated: output_graph.svg");
    } else {
        eprintln!(
            "Error generating SVG: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn main() {
    generate_graph();
}
