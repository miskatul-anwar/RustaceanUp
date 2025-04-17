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

    let mut i: i32 = 0;
    let t = sc.next();

    loop {
        if i == t {
            break;
        }

        let a: i32 = sc.next();
        let b: i32 = sc.next();
        let mut c: i32 = a;
        let mut tmp: i32 = 50;

        let min = loop {
            if c > b {
                break tmp;
            }

            tmp = tmp.min(c - a + b - c);
            c += 1
        };

        writeln!(out, "{}", min).unwrap();

        i += 1
    }
}
