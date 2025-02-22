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
fn same_parity(x: i32, y: i32) -> bool {
    if x % 2 != 0 && y % 2 != 0 || x % 2 == 0 && y % 2 == 0 {
        true
    } else {
        false
    }
}
fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let n = sc.next();
        let a: Vec<i32> = (0..n).map(|_| sc.next()).collect();
        let mut operations = 0;

        for i in 0..n - 1 {
            if same_parity(a[i], a[i + 1]) {
                operations += 1
            }
        }

        writeln!(out, "{}", operations).ok();
    }
}
