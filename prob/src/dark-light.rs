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
        let n: i32 = sc.next();
        let k: i32 = sc.next();

        if k == 0 {
            if n % 4 == 0 {
                writeln!(out, "OFF").unwrap()
            } else if n % 4 == 1 || n % 4 == 2 || n % 4 == 3 {
                writeln!(out, "ON").unwrap()
            } else {
                writeln!(out, "Ambiguous").unwrap()
            }
        } else {
            if n % 4 == 0 {
                writeln!(out, "ON").unwrap()
            } else {
                writeln!(out, "Ambiguous").unwrap()
            }
        }
    }
}
