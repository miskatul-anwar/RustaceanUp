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

fn is_universal(s: &str) -> bool {
    let n = s.len();
    for i in 0..n / 2 {
        if s.as_bytes()[i] != s.as_bytes()[n - 1 - i] {
            return s.as_bytes()[i] < s.as_bytes()[n - 1 - i];
        }
    }
    false
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let n: usize = sc.next();
        let k: i32 = sc.next();
        let s: String = sc.next();

        if n == 1 {
            writeln!(out, "NO").unwrap();
            continue;
        }

        let all_same = s.chars().all(|c| c == s.chars().next().unwrap());
        if all_same {
            writeln!(out, "NO").unwrap();
            continue;
        }

        if is_universal(&s) {
            writeln!(out, "YES").unwrap();
            continue;
        }

        writeln!(out, "{}", if k >= 1 { "YES" } else { "NO" }).unwrap();
    }
}
