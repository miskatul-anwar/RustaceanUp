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

fn reverse(mut s: String) -> String {
    let mut stack: Vec<char> = s.chars().into_iter().collect();
    s.clear();

    while let Some(top) = stack.pop() {
        s.push(top);
    }

    s
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let s = sc.next::<String>();

    writeln!(out, "{}", reverse(s)).unwrap()
}
