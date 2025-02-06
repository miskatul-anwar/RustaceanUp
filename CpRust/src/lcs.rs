use std::io::{self, BufRead};

struct Scanner {
    reader: Box<dyn BufRead>,
}

impl Scanner {
    fn new() -> Self {
        Self {
            reader: Box::new(io::stdin().lock()),
        }
    }

    fn _rin(&mut self) -> Vec<i32> {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input
            .split_whitespace()
            .map(|i| i.parse().expect("Failed to parse integer"))
            .collect()
    }

    fn _rin_int(&mut self) -> i32 {
        let mut input = String::new();
        self.reader.read_line(&mut input).unwrap();
        input.trim().parse().expect("Failed to parse integer")
    }
}

fn lcs(a: &[i32], b: &[i32]) -> Vec<i32> {
    let m = a.len() + 1;
    let n = b.len() + 1;

    // Create a 2D table to store the LCS lengths
    let mut dp = vec![vec![0; n]; m];

    // Fill in the table using dynamic programming
    for i in 1..m {
        for j in 1..n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    // Reconstruct the LCS from the table
    let mut lcs = Vec::new();
    let mut i = m - 1;
    let mut j = n - 1;

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            lcs.push(a[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    // Reverse the LCS to get the correct order
    lcs.reverse();

    lcs
}

fn main() {
    let mut scanner = Scanner::new();

    println!("Enter two integers separated by space:");
    let a: Vec<i32> = scanner._rin();
    let b: Vec<i32> = scanner._rin();

    if a.len() > 0 && b.len() > 0 {
        let lcs = lcs(&a, &b);
        println!("LCS: {:?}", lcs);
    } else {
        println!("Invalid input. Please enter two integers.");
    }
}
