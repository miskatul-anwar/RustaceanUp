#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Stdout, Write};

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

fn solve(scan: &mut Scanner, out: &mut BufWriter<Stdout>, sum: &mut usize) {
    let s: String = scan.next();

    match s.as_str() {
        "Tetrahedron" => *sum += 4,
        "Cube" => *sum += 6,
        "Octahedron" => *sum += 8,
        "Dodecahedron" => *sum += 12,
        "Icosahedron" => *sum += 20,
        _ => {}
    }
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut t: usize = scan.next();
    let mut sum: usize = 0;

    loop {
        if t == 0 {
            break;
        }

        solve(&mut scan, out, &mut sum);
        t -= 1
    }

    writeln!(out, "{}", sum).unwrap()
}
