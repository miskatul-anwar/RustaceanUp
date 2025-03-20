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

fn precompute(dp: &mut Vec<usize>) {
    for i in 1..1700 {
        if i % 3 == 0 || i % 10 == 3 {
            continue;
        }

        dp.push(i);
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let mut dp = Vec::new();
    precompute(&mut dp);

    let t = sc.next();
    for _ in 1..=t {
        let k: usize = sc.next();
        writeln!(out, "{}", dp[k - 1]).unwrap()
    }
}
