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

    if n == 0 {
        writeln!(out, "1").unwrap()
    } else if n % 4 == 0 {
        writeln!(out, "6").unwrap()
    } else if n % 4 == 1 {
        writeln!(out, "8").unwrap()
    } else if n % 4 == 2 {
        writeln!(out, "4").unwrap()
    } else if n % 4 == 3 {
        writeln!(out, "2").unwrap()
    }
}
