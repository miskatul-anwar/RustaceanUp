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

    let n: usize = sc.next();
    let mut tf = 0;
    let mut fy = 0;
    let mut possible = true;

    for _ in 0..n {
        let bill: i32 = sc.next();
        match bill {
            25 => tf += 1,
            50 => {
                if tf > 0 {
                    tf -= 1;
                    fy += 1;
                } else {
                    possible = false;
                    break;
                }
            }
            100 => {
                if fy > 0 && tf > 0 {
                    fy -= 1;
                    tf -= 1;
                } else if tf >= 3 {
                    tf -= 3;
                } else {
                    possible = false;
                    break;
                }
            }
            _ => {}
        }
    }

    writeln!(out, "{}", if possible { "YES" } else { "NO" }).unwrap();
}
