#![allow(unused)]
use std::{
    i32::MAX,
    io::{stdin, stdout, BufWriter, Write},
};

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

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp = vec![MAX; amount as usize + 1];
    dp[0] = 0;

    for a in 1..=amount {
        for &coin in &coins {
            if a >= coin && dp[(a - coin) as usize] != MAX {
                dp[a as usize] = dp[a as usize].min(dp[(a - coin) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] == MAX {
        -1
    } else {
        dp[amount as usize]
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let coins = (0..n).map(|_| sc.next()).collect();
    let amount = sc.next();
    let ans = coin_change(coins, amount);

    writeln!(out, "{}", ans).unwrap()
}
