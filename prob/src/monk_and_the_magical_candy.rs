#![allow(unused)]
use std::{
    collections::{BTreeMap, BTreeSet},
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
        let n = sc.next::<usize>();
        let mut k = sc.next::<i32>();
        let mut candies: Vec<i32> = (0..n).map(|_| sc.next()).collect();

        let mut consumed = 0;

        while k > 0 {
            candies.sort_unstable();
            consumed += candies[n - 1];
            candies[n - 1] /= 2;
            k -= 1
        }

        writeln!(out, "{consumed}").unwrap()
    }
}
