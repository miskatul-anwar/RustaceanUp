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

    let t = sc.next();
    for _ in 1..=t {
        let x = sc.next::<i64>();
        let n = sc.next::<i64>();

        let mut res = x;
        let mut s;

        if x % 2 != 0 {
            s = 1
        } else {
            s = -1
        }

        if n % 4 == 1 {
            res += s * n
        } else if n % 4 == 2 {
            res -= s
        } else if n % 4 == 3 {
            res -= s * (n + 1)
        }

        writeln!(out, "{}", res);
    }
}
