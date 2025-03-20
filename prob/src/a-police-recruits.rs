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

    let n = sc.next();
    let mut man_power = 0;
    let mut crimes = 0;
    let mut untreated = 0;

    for _ in 1..=n {
        let stat: i32 = sc.next();

        if stat > 0 {
            man_power += stat
        } else {
            if man_power < 1 {
                untreated += 1
            } else {
                man_power -= 1
            }
        }
    }

    writeln!(out, "{}", untreated).unwrap()
}
