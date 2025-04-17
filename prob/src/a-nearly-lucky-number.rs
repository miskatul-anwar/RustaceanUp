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

    let mut n: i64 = sc.next();
    let mut cnt: i32 = 0;

    while n > 0 {
        if n % 10 == 4 || n % 10 == 7 {
            cnt += 1
        }

        n /= 10
    }

    let mut nearly_lucky = true;

    if cnt <= 3 {
        nearly_lucky = false
    }

    while cnt > 0 {
        if !(cnt % 10 == 4 || cnt % 10 == 7) {
            nearly_lucky = false
        }

        cnt /= 10
    }

    if nearly_lucky {
        writeln!(out, "YES").unwrap()
    } else {
        writeln!(out, "NO").unwrap()
    }
}
