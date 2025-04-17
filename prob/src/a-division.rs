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
        let rating = sc.next();

        if rating >= 1900 {
            writeln!(out, "Division 1").unwrap()
        } else if 1600 <= rating && rating <= 1899 {
            writeln!(out, "Division 2").unwrap()
        } else if 1400 <= rating && rating <= 1599 {
            writeln!(out, "Division 3").unwrap()
        } else {
            writeln!(out, "Division 4").unwrap()
        }
    }
}
