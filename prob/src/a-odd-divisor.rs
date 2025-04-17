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

const N: u64 = 10e7 as u64;

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let n: u64 = sc.next();
        let mut found = false;

        if n & 1 == 1 {
            found = true
        } else {
            let mut div = n;

            while div > 2 {
                div /= 2;
                if div & 1 == 1 && n % div == 0 {
                    found = true;
                    break;
                }
            }
        }

        if found {
            writeln!(out, "YES").unwrap()
        } else {
            writeln!(out, "NO").unwrap()
        }
    }
}
