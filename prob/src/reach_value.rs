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
static mut FOUND: bool = false;
static mut TARGET: i64 = 0;

fn reach_value(n: i64) {
    if n == unsafe { TARGET } {
        unsafe { FOUND = true }
        return;
    }

    if n > unsafe { TARGET } {
        return;
    }

    reach_value(n * 10);
    reach_value(n * 20);
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let n = 1;
        unsafe { TARGET = sc.next() }

        reach_value(n);

        if unsafe { FOUND } {
            writeln!(out, "YES").unwrap()
        } else {
            writeln!(out, "NO").unwrap()
        }

        unsafe { FOUND = false }
    }
}
