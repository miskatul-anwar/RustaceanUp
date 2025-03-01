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
        let n: i64 = sc.next();
        let s: Vec<char> = sc.next::<String>().chars().into_iter().collect();

        let mut count: i64 = 1;
        let mut curr: i64 = 1;

        for p in 1..s.len() {
            if s[p] == s[p - 1] {
                curr += 1
            } else {
                curr = 1
            }

            count = count.max(curr)
        }

        writeln!(out, "{}", count + 1).unwrap()
    }
}
