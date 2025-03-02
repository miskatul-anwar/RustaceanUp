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
fn next_heigher_peak(heights: &mut Vec<i32>) {
    let mut max = -1;
    let mut curr_max = 0;

    for i in heights.iter_mut().rev() {
        curr_max = *i;

        if *i >= max {
            *i = -1
        } else {
            *i = max
        }

        if max < curr_max {
            max = curr_max
        }
    }
}
fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let mut heights: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    next_heigher_peak(&mut heights);

    for i in heights {
        write!(out, "{} ", i).unwrap()
    }

    writeln!(out, "").unwrap()
}
