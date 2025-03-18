use std::cmp::min;

fn binomial(n: usize, k: usize) -> usize {
    let mut dp = vec![vec![0; k + 1]; n + 1];

    for i in 0..=n {
        for j in 0..=min(i, k) {
            if j == 0 || j == i {
                dp[i][j] = 1
            } else {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j]
            }
        }
    }

    dp[n][k]
}

fn main() {
    let n: usize = 5;
    let k: usize = 3;
    println!("binomial of {n}, {k} = {}", binomial(n, k));
}
