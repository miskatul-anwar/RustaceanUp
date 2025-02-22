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
        let k = sc.next::<i64>();
        let x = sc.next::<i64>();

        let sum1 = (k * (k + 1)) / 2;
        let total = (n * (n + 1)) / 2;
        let diff = n - k;
        let sum2 = total - (diff * (diff + 1)) / 2;

        if x >= sum1 && x <= sum2 {
            writeln!(out, "YES").unwrap()
        } else {
            writeln!(out, "NO").unwrap()
        }
    }
}
