#![allow(unused)]
use std::{
    collections::VecDeque,
    io::{stdin, stdout, BufWriter, Write},
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

    let mut q: VecDeque<i32> = VecDeque::new();
    let t = sc.next();
    for _ in 1..=t {
        let mut x: i32 = sc.next();

        if x == 1 {
            x = sc.next();
            q.push_back(x);
        } else if x == 2 {
            if !q.is_empty() {
                q.pop_front();
            }
        } else {
            if let Some(front) = q.front() {
                writeln!(out, "{}", front).unwrap()
            } else {
                writeln!(out, "Empty!").unwrap()
            }
        }
    }
}
