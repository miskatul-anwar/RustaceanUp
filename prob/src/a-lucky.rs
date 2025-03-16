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

fn is_lucky(mut n: i32) -> bool {
    let mut first_half = 0;
    let mut second_half = 0;
    let mut idx = 5;

    while n > 0 {
        if idx >= 3 {
            second_half += n % 10;
            n /= 10
        } else {
            first_half += n % 10;
            n /= 10
        }

        idx -= 1
    }

    first_half == second_half
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();

    for _ in 1..=t {
        let mut n = sc.next();

        if is_lucky(n) {
            writeln!(out, "YES").unwrap()
        } else {
            writeln!(out, "NO").unwrap()
        }
    }
}
