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
fn even(indices: &Vec<i32>, n: i32) -> i32 {
    match n {
        -1 => 1,
        _ => {
            if n % 2 == 0 {
                print!("{} ", indices[n as usize]);
            }
            even(&indices, n - 1)
        }
    }
}
fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let indices: Vec<i32> = (0..n).map(|_| sc.next::<i32>()).collect();
    even(&indices, n - 1);
}
