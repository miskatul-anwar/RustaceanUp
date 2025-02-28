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

const MOD: i64 = 998244353;

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let vertex_count: usize = sc.next();

        let mut depth = vec![0; vertex_count + 1];
        depth[1] = 0;
        let mut max_depth = 0;

        for i in 2..=vertex_count {
            let parent: usize = sc.next();
            depth[i] = depth[parent] + 1;
            if depth[i] > max_depth {
                max_depth = depth[i];
            }
        }

        let mut depth_count = vec![0; max_depth + 1];
        for i in 2..=vertex_count {
            depth_count[depth[i]] += 1;
        }

        let mut sum = vec![0; max_depth + 2];
        sum[max_depth] = depth_count[max_depth] % MOD;

        for d in (1..max_depth).rev() {
            let ways = depth_count[d] % MOD;
            let factor = (depth_count[d] - 1) % MOD;
            sum[d] = (ways + (factor * sum[d + 1]) % MOD) % MOD;
        }

        let result = (1 + sum[1]) % MOD;
        writeln!(out, "{}", result).unwrap();
    }
}
