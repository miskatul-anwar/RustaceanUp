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

    let s: String = sc.next();
    let mut ans = String::new();
    let mut i = 0;

    while i < s.len() {
        if i + 2 < s.len() && &s[i..i + 3] == "WUB" {
            if !ans.is_empty() && ans.chars().last().unwrap() != ' ' {
                ans.push(' ');
            }
            i += 3;
        } else {
            ans.push(s.chars().nth(i).unwrap());
            i += 1;
        }
    }

    writeln!(out, "{}", ans).unwrap();
}
