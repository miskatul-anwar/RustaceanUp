fn cut_rod(price: &Vec<i32>) -> i32 {
    let n = price.len();
    let mut dp: Vec<i32> = vec![0; n + 1];

    for i in 1..=n {
        for j in 1..=i {
            dp[i] = dp[i].max(price[j - 1] + dp[i - j])
        }
    }

    dp[n]
}

fn main() {
    let price = vec![1, 5, 8, 9, 10, 17, 17, 20];
    println!("{}", cut_rod(&price))
}
