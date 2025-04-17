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
        let mut n: i32 = sc.next();
        let mut steps = 0;

        while n % 6 == 0 {
            n /= 6;
            steps += 1
        }

        while n % 2 == 0 {
            n /= 2;
            steps += 1
        }

        writeln!(out, "{}", steps).unwrap()
    }
}
