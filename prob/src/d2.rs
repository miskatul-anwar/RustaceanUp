use std::io::{stdout, BufWriter, Write};

fn main() {
    let out = &mut BufWriter::new(stdout());
    writeln!(out, "0.000000 0.000000").unwrap()
}
