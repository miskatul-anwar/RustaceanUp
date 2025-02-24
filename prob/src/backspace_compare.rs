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
fn backspace_compare(mut s: String, mut t: String) -> bool {
    fn process(input: String) -> String {
        let mut stack = Vec::new();

        for c in input.chars() {
            if c == '#' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack.into_iter().collect()
    }

    process(s) == process(t)
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let s = sc.next::<String>();
    let t = sc.next::<String>();

    if backspace_compare(s, t) {
        writeln!(out, "YES").unwrap()
    } else {
        writeln!(out, "NO").unwrap()
    }
}
