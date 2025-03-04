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

    let t = sc.next();
    for _ in 1..=t {
        let w: String = sc.next();
        let p: usize = sc.next();
        let mut pr = 0;

        for ch in w.chars() {
            pr += ch as usize - 96
        }

        if pr <= p {
            writeln!(out, "{}", w).unwrap();
            continue;
        }

        let mut w: Vec<char> = w.chars().into_iter().collect();

        for ch in w {
            write!(out, "{}", ch).unwrap()
        }

        writeln!(out).unwrap()
    }
}
// FIXME
