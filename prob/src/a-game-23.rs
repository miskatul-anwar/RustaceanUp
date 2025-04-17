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

    let n: i32 = sc.next();
    let m: i32 = sc.next();

    if m % n != 0 {
        writeln!(out, "-1").unwrap()
    } else {
        let mut moves = 0;
        let mut x = m / n;

        while x % 2 == 0 {
            x /= 2;
            moves += 1
        }

        while x % 3 == 0 {
            x /= 3;
            moves += 1
        }

        if x == 1 {
            writeln!(out, "{}", moves).unwrap()
        } else {
            writeln!(out, "-1").unwrap()
        }
    }
}
