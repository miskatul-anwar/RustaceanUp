#![allow(unused)]
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Write},
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct Pair<T: PartialEq + PartialOrd> {
    first: T,
    second: T,
}

impl<T: PartialEq + PartialOrd> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair {
            first: x,
            second: y,
        }
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: i32 = sc.next();
    let m: i32 = sc.next();
    let mut a: VecDeque<Pair<i32>> = VecDeque::new();

    for i in 1..=n {
        let p: Pair<i32> = Pair::new(i, sc.next());
        a.push_back(p);
    }

    let mut idx: i32 = 0;

    while let Some(mut front) = a.pop_front() {
        if front.second > m {
            front.second -= m;
            a.push_back(front);
        }

        idx = front.first
    }

    writeln!(out, "{}", idx).unwrap()
}
