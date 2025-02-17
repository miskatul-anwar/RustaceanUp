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

    let t = sc.next();
    for _ in 1..=t {
        let n = sc.next::<i32>();

        let s: Vec<char> = sc.next::<String>().chars().into_iter().collect();

        let (mut l, mut r) = (0, n - 1);

        while l < r && s[l as usize] != s[r as usize] {
            l += 1;
            r -= 1;
        }

        writeln!(out, "{} ", r - l + 1);
    }
}
