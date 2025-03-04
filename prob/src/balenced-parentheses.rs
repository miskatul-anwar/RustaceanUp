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
type Stack<T> = Vec<T>;

fn par_match(open: char, close: char) -> bool {
    let openers = "([{";
    let closers = ")]}";
    openers.find(open) == closers.find(close)
}
fn balenced_parentheses(s: &str) -> bool {
    let mut stack = Stack::new();

    for c in s.chars() {
        if c == '(' || c == '{' || c == '[' {
            stack.push(c);
        } else if let Some(top) = stack.pop() {
            if !par_match(top, c) {
                return false;
            }
        } else {
            return false;
        }
    }

    stack.is_empty()
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let s: String = sc.next();

    if balenced_parentheses(&s) {
        writeln!(out, "YES").unwrap()
    } else {
        writeln!(out, "NO").unwrap()
    }
}
