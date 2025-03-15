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

    let (n, h) = (sc.next::<usize>(), sc.next::<i32>());
    let a: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    let mut width = 0;
    for i in 0..n {
        if a[i] > h {
            width += 2
        } else {
            width += 1
        }
    }

    writeln!(out, "{}", width).unwrap()
}
