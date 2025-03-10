#![allow(unused)]
use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Write},
    iter::Sum,
};

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
    let nsum = n * (n + 1) / 2;
    let mut hashset: HashSet<usize> = HashSet::new();

    let p = sc.next();

    for _ in 0..p {
        hashset.insert(sc.next());
    }

    let q = sc.next();

    for _ in 0..q {
        hashset.insert(sc.next());
    }

    let sum: usize = hashset.iter().sum();

    if sum == nsum {
        writeln!(out, "I become the guy.").unwrap()
    } else {
        writeln!(out, "Oh, my keyboard!").unwrap()
    }
}
