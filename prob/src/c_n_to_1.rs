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

fn print(n: i32) {
    match n {
        1 => println!("{n}"),
        _ => {
            print!("{n} ");
            print(n - 1);
        }
    }
}
fn main() {
    let mut sc = Scanner::default();

    let t = sc.next();
    for _ in 1..=t {
        let n = sc.next();
    }
    print(n);
}
