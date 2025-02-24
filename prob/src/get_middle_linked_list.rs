#![allow(unused)]
use std::{
    collections::LinkedList,
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

    let size = sc.next();
    let nums: LinkedList<_> = (0..size).map(|_| sc.next::<i32>()).collect();
    //FIXME

    let mut n = 0;
    for &num in nums.iter() {
        if n >= nums.len() / 2 {
            write!(out, "{num} ").unwrap()
        }
        n += 1
    }

    writeln!(out, "").unwrap()
}
