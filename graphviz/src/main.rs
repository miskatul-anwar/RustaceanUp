use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{attributes::*, cmd::Format, exec, parse, printer::PrinterContext};
use std::fs::File;
use std::io::Write;

fn parse_test() {
    let dot_data = r#"
        strict digraph t {
            aa[color=green]
            subgraph v {
                aa[shape=square]
                subgraph vv{a2 -> b2}
                aaa[color=red]
                aaa -> bbb
            }
            aa -> be -> subgraph v { d -> aaa}
            aa -> aaa -> v
        }
    "#;

    match parse(dot_data) {
        Ok(g) => {
            println!("Parsed The Graph Successfully!: {:?}", g);
        }
        Err(e) => {
            eprintln!("Error parsing DOT data: {:?}", e);
        }
    }
}

fn output_test() {
    let g = graph!(id!("id");
         node!("nod"),
         subgraph!("sb";
             edge!(node_id!("a") => subgraph!(;
                node!("n";
                NodeAttributes::color(color_name::black), NodeAttributes::shape(shape::egg))
            ))
        ),
        edge!(node_id!("a1") => node_id!(esc "a2"))
    );

    let graph_svg = exec(g, &mut PrinterContext::default(), vec![Format::Svg.into()]).unwrap();

    let mut file = File::create("output_graph.svg").expect("Failed to create file");
    file.write_all(&graph_svg).expect("Failed to write to file");
    println!("Graph saved as output_graph.svg");
}

fn main() {
    parse_test();
    output_test();
}
