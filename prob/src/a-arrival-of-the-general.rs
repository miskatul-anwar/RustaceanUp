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
    let a: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    let mut min_value = i32::MAX;
    let mut max_value = i32::MIN;
    let mut min_pos = 0;
    let mut max_pos = 0;

    for i in 0..n {
        if a[i] > max_value {
            max_value = a[i];
            max_pos = i;
        }
        if a[i] <= min_value {
            min_value = a[i];
            min_pos = i;
        }
    }

    let req_time = if max_pos > min_pos {
        max_pos + (n - min_pos - 2)
    } else {
        max_pos + (n - min_pos - 1)
    };

    writeln!(out, "{}", req_time).unwrap();
}
