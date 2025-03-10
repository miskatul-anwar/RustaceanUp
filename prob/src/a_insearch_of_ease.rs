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

    let t: i8 = sc.next();
    let mut is_easy = true;

    for _ in 1..=t {
        let opinion: i8 = sc.next();

        if opinion == 1 {
            is_easy = false
        }
    }

    if is_easy {
        writeln!(out, "EASY").unwrap()
    } else {
        writeln!(out, "HARD").unwrap()
    }
}
