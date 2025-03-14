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

    let test_cases: i32 = sc.next();

    for _ in 0..test_cases {
        let mut n: i32 = sc.next();
        let mut numbers = Vec::new();

        if n >= 1000 {
            let rem = n % 1000;
            numbers.push(n - rem);
            n %= 1000;
        }
        if n >= 100 {
            let rem = n % 100;
            numbers.push(n - rem);
            n %= 100;
        }
        if n >= 10 {
            let rem = n % 10;
            numbers.push(n - rem);
            n %= 10;
        }
        if n > 0 {
            numbers.push(n);
        }

        writeln!(out, "{}", numbers.len()).unwrap();
        for num in &numbers {
            write!(out, "{} ", num).unwrap();
        }
        writeln!(out).unwrap();
    }
}
