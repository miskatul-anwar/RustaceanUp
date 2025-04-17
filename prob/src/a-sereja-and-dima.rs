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
    let mut cards = vec![0; n + 1];

    for i in 1..=n {
        cards[i] = sc.next();
    }

    let mut i = 1;
    let mut j = n;

    let mut sum1 = 0;
    let mut sum2 = 0;

    let mut k = 1;

    while i <= j {
        if k % 2 == 1 {
            if cards[i] >= cards[j] {
                sum1 += cards[i];
                i += 1
            } else {
                sum1 += cards[j];
                j -= 1
            }
        } else {
            if cards[i] >= cards[j] {
                sum2 += cards[i];
                i += 1
            } else {
                sum2 += cards[j];
                j -= 1
            }
        }
        k += 1
    }

    writeln!(out, "{} {}", sum1, sum2).unwrap()
}
