fn lucas_numbers(dp: &mut Vec<i32>) {
    dp[0] = 2;
    dp[1] = 1;

    for i in 2..dp.len() {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
}

fn main() {
    let n = 6;
    let mut dp = vec![0; n + 1];
    lucas_numbers(&mut dp);

    println!("{:?} ", dp);
}
