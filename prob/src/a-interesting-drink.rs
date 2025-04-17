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
    let mut cost: Vec<usize> = vec![0; 100000];
    let mut maxx: usize = 0;

    for i in 0..n {
        let x: usize = sc.next();
        cost[x] += 1;

        maxx = maxx.max(x)
    }

    for i in 1..=maxx {
        cost[i] += cost[i - 1]
    }

    let mut q: usize = sc.next();
    let mut result: Vec<String> = Vec::new();

    while q > 0 {
        let m: usize = sc.next();

        if m >= maxx {
            result.push(n.to_string());
        } else {
            result.push(cost[m].to_string());
        }

        q -= 1
    }

    writeln!(out, "{}", result.join("\n")).unwrap()
}
