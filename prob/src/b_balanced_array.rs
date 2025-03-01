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
        let n: usize = sc.next();

        if n % 4 == 0 {
            writeln!(out, "YES").unwrap();

            let mut sum1 = 0;
            let mut sum2 = 0;

            for i in (2..=n).step_by(2) {
                write!(out, "{} ", i).unwrap();
                sum1 += i
            }

            for i in (1..n - 2).step_by(2) {
                write!(out, "{} ", i).unwrap();
                sum2 += i
            }

            writeln!(out, "{}", sum1 - sum2).unwrap()
        } else {
            writeln!(out, "NO").unwrap()
        }
    }
}
