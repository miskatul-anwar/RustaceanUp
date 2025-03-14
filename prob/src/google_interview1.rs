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
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();

    let mut dp = vec![vec![0; n + 1]; m + 1];
    let mut mx = 0;

    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == '1' {
                dp[i + 1][j + 1] = (dp[i][j + 1].min(dp[i + 1][j])).min(dp[i][j]) + 1;
                mx = mx.max(dp[i + 1][j + 1])
            }
        }
    }

    mx * mx
}
fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());
}
