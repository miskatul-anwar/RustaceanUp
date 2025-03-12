#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Write};

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

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let s: String = sc.next();

        match s.as_str() {
            "YES" => writeln!(out, "YES").unwrap(),
            "Yes" => writeln!(out, "YES").unwrap(),
            "yEs" => writeln!(out, "YES").unwrap(),
            "yeS" => writeln!(out, "YES").unwrap(),
            "yes" => writeln!(out, "YES").unwrap(),
            "YeS" => writeln!(out, "YES").unwrap(),
            "YEs" => writeln!(out, "YES").unwrap(),
            "yES" => writeln!(out, "YES").unwrap(),
            _ => writeln!(out, "NO").unwrap(),
        }
    }
}
