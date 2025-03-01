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

fn compute_value(m: i64, n: i64, base: i32, prefix_xor: &Vec<i32>) -> i32 {
    if m <= n {
        return prefix_xor[m as usize];
    }
    if m & 1 == 1 {
        base
    } else {
        base ^ compute_value(m / 2, n, base, prefix_xor)
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let n: i64 = sc.next();
        let left: i64 = sc.next();
        let right: i64 = sc.next();

        let mut array = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            array[i] = sc.next();
        }

        if left <= n {
            writeln!(out, "{}", array[left as usize]).unwrap();
            continue;
        }

        let mut prefix_xor = vec![0; n as usize + 1];
        for i in 1..=n as usize {
            prefix_xor[i] = prefix_xor[i - 1] ^ array[i];
        }

        let full_xor = prefix_xor[n as usize];
        let base = if n & 1 == 1 {
            full_xor
        } else {
            full_xor ^ prefix_xor[(n / 2) as usize]
        };

        let m = left / 2;
        let result = compute_value(m, n, base, &prefix_xor);
        writeln!(out, "{}", result).unwrap();
    }
}
