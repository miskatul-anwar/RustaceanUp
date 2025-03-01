const N: usize = 100 as usize;

fn precompute(dp: &mut Vec<i128>) {
    dp.push(0);
    dp.push(1);

    for i in 2..N {
        dp.push(dp[i - 1] + dp[i - 2])
    }
}

fn main() {
    let mut dp = Vec::new();
    precompute(&mut dp);

    println!("Fibonacci of {} is: {}", 80, dp[80 - 1])
}
