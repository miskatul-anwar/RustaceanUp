#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Write};

const MOD: i64 = 998244353;
const MAX_N: usize = 500_000;

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

struct Factorials {
    fact: Vec<i64>,
    inv_fact: Vec<i64>,
}

impl Factorials {
    fn new() -> Self {
        let mut fact = vec![1; MAX_N + 1];
        for i in 1..=MAX_N {
            fact[i] = (fact[i - 1] * i as i64) % MOD;
        }

        let mut inv_fact = vec![1; MAX_N + 1];
        inv_fact[MAX_N] = mod_exp(fact[MAX_N], MOD - 2);
        for i in (1..=MAX_N).rev() {
            inv_fact[i - 1] = (inv_fact[i] * i as i64) % MOD;
        }

        Self { fact, inv_fact }
    }
}

fn mod_exp(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % MOD;
        }
        base = (base * base) % MOD;
        exp >>= 1;
    }
    res
}

fn count_subsets(counts: &[i32], target: i32) -> i64 {
    let mut dp = vec![0; target as usize + 1];
    dp[0] = 1;
    for &val in counts {
        let val = val as usize;
        for j in (val..=target as usize).rev() {
            dp[j] = (dp[j] + dp[j - val]) % MOD;
        }
    }
    dp[target as usize]
}

fn main() {
    let factorials = Factorials::new();
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let mut char_counts = [0; 26];
        let mut total_len = 0;
        for i in 0..26 {
            char_counts[i] = sc.next();
            total_len += char_counts[i] as i64;
        }

        let odd_positions = (total_len + 1) / 2;
        let even_positions = total_len / 2;

        if char_counts.iter().any(|&c| c as i64 > odd_positions) {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut forced_sum = 0;
        let mut free_counts = Vec::new();
        for &count in &char_counts {
            if count == 0 {
                continue;
            }
            if count as i64 > even_positions {
                forced_sum += count;
            } else {
                free_counts.push(count);
            }
        }

        let target = (odd_positions - forced_sum as i64) as i32;
        if target < 0 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let free_total: i32 = free_counts.iter().sum();
        if target > free_total {
            writeln!(out, "0").unwrap();
            continue;
        }

        let subset_ways = count_subsets(&free_counts, target);
        let mut arrangement = (factorials.fact[odd_positions as usize]
            * factorials.fact[even_positions as usize])
            % MOD;
        for &count in &char_counts {
            if count > 0 {
                arrangement = (arrangement * factorials.inv_fact[count as usize]) % MOD;
            }
        }

        let result = (subset_ways * arrangement) % MOD;
        writeln!(out, "{}", result).unwrap();
    }
}
