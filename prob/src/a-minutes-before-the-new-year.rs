#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Stdout, Write};

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
    let h = scan.next::<i16>();
    let m = scan.next::<i16>();

    let ans = (24 - (h + 1)) * 60 + (60 - m);
    writeln!(out, "{}", ans).unwrap()
}

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let mut t = scan.next::<i16>();

    loop {
        if t == 0 {
            break;
        }

        solve(&mut scan, out);
        t -= 1
    }
}
