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
        let n = sc.next();
        let mut q: VecDeque<i32> = (0..n).map(|_| sc.next()).collect();
        // let mut reversed = VecDeque::new();

        // while let Some(front) = q.pop_front() {
        //     reversed.push_front(front);
        // }

        // for i in reversed {
        //     write!(out, "{} ", i).ok();
        // }

        q.make_contiguous().reverse();
        writeln!(out, "{:?}", q).ok();
    }
}
