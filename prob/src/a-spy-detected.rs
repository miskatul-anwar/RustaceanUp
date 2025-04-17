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
        let n: usize = sc.next();
        let a: Vec<usize> = (0..n).map(|_| sc.next()).collect();

        let mut i = 1;

        let ans = loop {
            if a[i] != a[i - 1] {
                if i == 1 {
                    if a[i - 1] == a[i + 1] {
                        break 2;
                    } else {
                        break 1;
                    }
                }

                break i + 1;
            }

            i += 1
        };

        writeln!(out, "{}", ans).unwrap()
    }
}
