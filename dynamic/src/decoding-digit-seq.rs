fn count_decodes(digits: &str) -> usize {
    let n = digits.len();
    let mut dp = vec![0; n + 1];

    dp[n] = 1;
    for i in (0..n).rev() {
        if digits.chars().nth(i).unwrap() == '0' {
            dp[i] = 0;
        } else {
            dp[i] = dp[i + 1];
            if i + 1 < n
                && (digits.chars().nth(i).unwrap() == '1'
                    || (digits.chars().nth(i).unwrap() == '2'
                        && digits.chars().nth(i + 1).unwrap() < '7'))
            {
                dp[i] += dp[i + 2];
            }
        }
    }

    dp[0]
}

fn main() {
    let digits = "121";
    println!("{}", count_decodes(digits));
}
