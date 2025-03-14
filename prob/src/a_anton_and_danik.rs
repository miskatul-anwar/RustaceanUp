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
    let s: String = sc.next();

    let mut anton = 0;
    let mut danik = 0;

    for w in s.chars().into_iter() {
        if w == 'A' {
            anton += 1
        } else {
            danik += 1
        }
    }

    if anton > danik {
        writeln!(out, "Anton").unwrap()
    } else if danik > anton {
        writeln!(out, "Danik").unwrap()
    } else {
        writeln!(out, "Friendship").unwrap()
    }
}
