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

    let n: usize = sc.next();
    let t: usize = sc.next();

    if n == 1 && t == 10 {
        writeln!(out, "-1").unwrap()
    } else if n >= 2 && t == 10 {
        for i in 1..n {
            write!(out, "1").unwrap()
        }

        writeln!(out, "0").unwrap()
    } else {
        for i in 1..=n {
            write!(out, "{}", t).unwrap()
        }

        writeln!(out).unwrap()
    }
}
