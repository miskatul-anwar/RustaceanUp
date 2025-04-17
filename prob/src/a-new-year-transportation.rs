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

    let n: usize = sc.next();
    let t: usize = sc.next();

    let a: Vec<usize> = (0..n - 1).map(|_| sc.next()).collect();

    let mut possible: bool = false;
    let mut i: usize = 0;

    while i < n - 1 {
        i += a[i];

        if i + 1 == t {
            possible = true;
            break;
        }
    }

    if possible {
        writeln!(out, "YES").unwrap()
    } else {
        writeln!(out, "NO").unwrap()
    }
}
