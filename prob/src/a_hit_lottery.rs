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

    const NOTES: [usize; 5] = [100, 20, 10, 5, 1];
    let mut ans: usize = 0;
    let mut n: usize = sc.next();

    for i in 0..5 {
        ans += n / NOTES[i];
        n = n % NOTES[i]
    }

    writeln!(out, "{}", ans).unwrap()
}
