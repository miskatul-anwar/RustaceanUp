#![allow(unused)]
use core::num;
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
        let n = sc.next::<usize>();
        let nums: Vec<i32> = (0..n).map(|_| sc.next()).collect();
        let mut tracked = false;

        for i in 0..n - 1 {
            for j in i..n - 1 {
                if nums[j] > nums[i] && !tracked {
                    write!(out, "{} ", nums[j]).unwrap();
                    tracked = true
                }
            }

            if !tracked {
                write!(out, "-1 ").unwrap()
            }
        }

        writeln!(out, "-1 ").unwrap()
    }
}
