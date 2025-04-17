#![allow(unused)]
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufWriter, Write},
    vec,
};

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
    let m: usize = sc.next();
    let mut map = HashMap::new();

    for _ in 1..=m {
        let a: String = sc.next();
        let b: String = sc.next();

        if a.len() > b.len() {
            map.insert(a, b);
        } else {
            map.insert(b, a);
        }
    }

    let mut output = Vec::new();

    for i in 0..n {
        let word: String = sc.next();

        match map.get(&word) {
            Some(value) => output.push(value.clone()),
            None => output.push(word),
        }

        write!(out, "{} ", output[i]).unwrap()
    }

    writeln!(out).unwrap()
}
