/*https://atcoder.jp/contests/dp/tasks/dp_b*/
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

fn min_cost(i: usize, k: usize, h: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
    if i == 0 {
        return 0;
    }

    if dp[i as usize] != -1 {
        return dp[i];
    }

    let mut cost = i32::MAX;
    for j in 1..=k {
        if i >= j {
            cost = cost.min(min_cost(i - j, k, h, dp) + (h[i] - h[i - j]).abs());
        }
    }
    dp[i] = cost;
    cost
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n: usize = sc.next();
    let k: usize = sc.next();
    let h: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let mut dp = vec![-1; n]; // ✅ Fix large array issue

    writeln!(out, "{}", min_cost(n - 1, k, &h, &mut dp)).unwrap();
}
