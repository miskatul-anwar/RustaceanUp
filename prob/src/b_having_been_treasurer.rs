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
        let n = sc.next::<i64>();
        let s = sc.next::<String>();

        let (mut m, mut u) = (0i64, 0i64);
        for c in s.chars() {
            if c == '-' {
                m += 1
            } else {
                u += 1
            }
        }

        if m < 2 || u == 0 {
            writeln!(out, "0").unwrap()
        } else {
            writeln!(out, "{}", m * m / 4).unwrap()
        }
    }
}
