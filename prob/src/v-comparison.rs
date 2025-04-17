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

    let a: i32 = sc.next();
    let op: char = sc.next();
    let b: i32 = sc.next();
    let mut ok = false;

    if op == '>' && a > b {
        ok = true
    } else if op == '<' && a < b {
        ok = true
    } else if op == '=' && a == b {
        ok = true
    }

    if ok {
        writeln!(out, "Right").unwrap()
    } else {
        writeln!(out, "Wrong").unwrap()
    }
}
