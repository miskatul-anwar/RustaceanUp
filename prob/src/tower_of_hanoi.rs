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
fn hanoi(from: u32, aux: u32, to: u32, n: u32) {
    if n == 0 {
        return;
    }

    hanoi(from, to, aux, n - 1);
    println!("{} {}", from, to);
    hanoi(aux, from, to, n - 1);
}

fn main() {
    let mut sc = Scanner::default();

    let n: u32 = sc.next();
    println!("{}", (1 << n) - 1); // 2^n - 1 moves required. (1 << n) == 2^n

    hanoi(1, 2, 3, n);
}
