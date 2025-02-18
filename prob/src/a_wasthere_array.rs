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
        let n = sc.next::<usize>() - 2;
        let b: Vec<i32> = (0..n).map(|_| sc.next()).collect();
        let mut possible = true;

        if n <= 2 {
            writeln!(out, "YES").ok();
            continue;
        }

        for i in 0..n - 2 {
            if b[i] == 1 && b[i + 1] == 0 && b[i + 2] == 1 {
                possible = false;
                break;
            }
        }

        if possible {
            writeln!(out, "YES").ok();
        } else {
            writeln!(out, "NO").ok();
        }
    }
}
