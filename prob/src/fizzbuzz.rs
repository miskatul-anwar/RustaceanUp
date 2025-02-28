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

fn calculate_fizzbuzz_count(max_number: i64) -> i64 {
    let full_blocks = max_number / 15;
    let mut count = full_blocks * 3;

    let remainder = max_number % 15;
    for i in 0..=remainder {
        if i % 3 == i % 5 {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let max_number: i64 = sc.next();
        let result = calculate_fizzbuzz_count(max_number);
        writeln!(out, "{}", result).unwrap();
    }
}
