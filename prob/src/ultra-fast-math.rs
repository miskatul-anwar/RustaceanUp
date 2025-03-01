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

    let num1: Vec<char> = sc.next::<String>().chars().into_iter().collect();
    let num2: Vec<char> = sc.next::<String>().chars().into_iter().collect();

    let mut ans = String::new();

    for (d1, d2) in num1.iter().zip(&num2) {
        if d1 != d2 {
            ans.push('1')
        } else {
            ans.push('0')
        }
    }

    writeln!(out, "{}", ans).expect("Failed to write!!")
}
