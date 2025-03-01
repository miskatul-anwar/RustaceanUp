#![allow(unused)]
use std::{
    io::{stdin, stdout, BufWriter, Write},
    mem::swap,
};

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
        let mut x: i64 = sc.next();
        let mut y: i64 = sc.next();
        let a: i64 = sc.next();
        let b: i64 = sc.next();

        if x > y {
            swap(&mut x, &mut y);
        }

        let ans1 = x * a + y * a;
        let ans2 = (y - x) * a + x * b;

        writeln!(out, "{}", ans1.min(ans2)).unwrap()
    }
}
