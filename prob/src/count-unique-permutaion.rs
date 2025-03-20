//It can be confirmed that given string doesn't have any repeating characters
fn count_valid(n: usize) -> usize {
    let mut dp = vec![1; n + 1];

    for i in 2..=n {
        dp[i] = dp[i - 1] * i;
    }

    dp[n]
}

fn main() {
    let string = "ABCD";
    let n = string.len();
    println!("{}", count_valid(n))
}
