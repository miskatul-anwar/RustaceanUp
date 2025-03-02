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
                return token.parse().ok().expect("Failed to parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

// Josephus recursive function
fn josephus(n: usize, k: usize) -> usize {
    if n == 1 {
        0
    } else {
        (josephus(n - 1, k) + k) % n
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t: usize = sc.next(); // Number of test cases

    for _ in 0..t {
        let n: usize = sc.next();
        let k: usize = sc.next();

        // Josephus returns 0-based index, so add 1 for 1-based output
        let result = josephus(n, k) + 1;
        writeln!(out, "{}", result).unwrap();
    }
}
