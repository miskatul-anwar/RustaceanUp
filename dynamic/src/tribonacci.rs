fn tribonacci(n: usize, dp: &mut Vec<i32>) {
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = 1;

    for i in 3..n {
        dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
    }
}

fn main() {
    let n: usize = 5;
    let mut dp = vec![0; n + 1];
    tribonacci(n, &mut dp);

    for i in 0..n {
        print!("{} ", dp[i])
    }

    println!()
}
