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

fn is_perfect_square(x: i64) -> bool {
    if x < 0 {
        return false;
    }
    let r = (x as f64).sqrt().round() as i64;
    r * r == x
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let n: i32 = sc.next();
        let total = (n as i64) * (n as i64 + 1) / 2;

        if is_perfect_square(total) {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let mut permutation: Vec<i32> = (1..=n).collect();
        let mut prefix_sum = 0;

        for i in 0..n - 1 {
            if is_perfect_square(prefix_sum + permutation[i as usize] as i64) {
                permutation.swap(i as usize, (i + 1) as usize);
            }
            prefix_sum += permutation[i as usize] as i64;
        }

        for i in 0..n {
            write!(out, "{}", permutation[i as usize]).unwrap();
            if i + 1 < n {
                write!(out, " ").unwrap();
            }
        }
        writeln!(out).unwrap();
    }
}
