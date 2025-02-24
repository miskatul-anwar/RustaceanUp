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

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let s = sc.next::<String>();

        let mut ones = 0;
        let mut zeros = 0;

        for i in s.chars() {
            if i == '1' {
                ones += 1
            } else {
                zeros += 1
            }
        }

        let ans = ones.min(zeros);

        if ans % 2 != 0 {
            writeln!(out, "DA").unwrap()
        } else {
            writeln!(out, "NET").unwrap()
        }
    }
}
