fn pascals_triangle(n: usize) {
    let mut dp = vec![vec![0; n]; n];
    dp[0][0] = 1;

    for i in 1..n {
        dp[i][0] = 1;
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
        }
    }

    for i in 0..n {
        for j in 0..=i {
            print!("{} ", dp[i][j]);
        }
        println!();
    }
}

fn main() {
    pascals_triangle(5);
}
