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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct Pair<T: PartialEq + PartialOrd> {
    first: T,
    second: T,
}

impl<T: PartialEq + PartialOrd> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            first: x,
            second: y,
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: usize = sc.next();
    let mut a: Vec<Pair<usize>> = vec![Pair::new(0, 0); n];

    for i in 0..n {
        a[i].first = sc.next();
        a[i].second = sc.next();
    }

    a.sort_unstable();

    for i in 0..n - 1 {
        if a[i].first < a[i + 1].first && a[i].second > a[i + 1].second {
            writeln!(out, "Happy Alex").unwrap();
            return;
        }
    }

    writeln!(out, "Poor Alex").unwrap()
}
