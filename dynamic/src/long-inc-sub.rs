// Longest Increasing Subsequence
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

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![-1; nums.len()];

    fn lis(i: usize, nums: &Vec<i32>, dp: &mut Vec<i32>) -> i32 {
        if dp[i] != -1 {
            return dp[i];
        }

        let mut ans = 1;

        for j in 0..i {
            if nums[i] > nums[j] {
                ans = ans.max(lis(j, nums, dp) + 1)
            }
        }

        dp[i] = ans;
        ans
    }

    let mut ans = 0;

    for i in 0..nums.len() {
        ans = ans.max(lis(i, &nums, &mut dp))
    }

    ans
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let nums: Vec<_> = (0..n).map(|_| sc.next::<i32>()).collect();

    writeln!(out, "{}", length_of_lis(nums)).unwrap()
}
