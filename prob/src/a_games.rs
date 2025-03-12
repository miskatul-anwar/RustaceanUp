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

    let mut h = Vec::new();
    let mut a = Vec::new();

    for _ in 1..=t {
        h.push(sc.next::<i32>());
        a.push(sc.next::<i32>());
    }

    let mut count = 0;

    for &i in h.iter() {
        for &j in a.iter() {
            if i == j {
                count += 1
            }
        }
    }

    writeln!(out, "{}", count).unwrap()
}
