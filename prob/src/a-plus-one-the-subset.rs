#![allow(unused)]
use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Stdout, Write},
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

fn solve(scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
    let n: usize = scan.next();
    let mut s: HashSet<i32> = (0..n).map(|_| scan.next()).collect();
    let max = *s.iter().max().unwrap();
    let mut steps = 0;

    while s.len() > 1 {
        let mut new_s = HashSet::new();

        for &i in &s {
            if i == max {
                new_s.insert(i + 1);
            } else {
                new_s.insert(i);
            }
        }

        s = new_s;
        steps += 1;
    }

    writeln!(out, "{}", steps).unwrap()
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut t: usize = scan.next();

    loop {
        if t == 0 {
            break;
        }

        solve(&mut scan, out);
        t -= 1
    }
}
