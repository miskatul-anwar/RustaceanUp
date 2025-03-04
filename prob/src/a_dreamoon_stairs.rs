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

    let n: i32 = sc.next();
    let m: i32 = sc.next();
    let mut steps: i32 = 0;

    if m > n {
        steps = -1
    } else {
        if n % 2 == 0 {
            steps = n / 2
        } else {
            steps = n / 2 + 1
        }

        while steps % m != 0 {
            steps += 1
        }
    }

    writeln!(out, "{}", steps).unwrap()
}
