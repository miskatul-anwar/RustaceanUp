use std::cmp::max;

fn lcs(s1: &str, s2: &str) -> (usize, String) {
    let m = s1.len();
    let n = s2.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    let mut lcs = String::new();

    let mut i = m;
    let mut j = n;

    while i > 0 && j > 0 {
        if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
            // each time push at the front on the string
            lcs.insert(0, s1.chars().nth(i - 1).unwrap());

            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    (dp[m][n], lcs)
}

fn main() {
    let s1 = "AGGTAB";
    let s2 = "GXTXAYB";

    let (size, string) = lcs(s1, s2);

    println!("{} {}", size, string)
}
