// This problem is known as MCM
use std::cmp::min;
use std::usize::MAX;

fn matrix_multiplication(arr: &Vec<usize>) -> usize {
    let n = arr.len();
    let mut dp = vec![vec![0; n]; n];

    for l in 2..n {
        for i in 0..n - l {
            let j = i + l;

            dp[i][j] = MAX;
            for k in i + 1..j {
                let cost = dp[i][k] + dp[k][j] + arr[i] * arr[k] * arr[j];
                dp[i][j] = min(dp[i][j], cost);
            }
        }
    }

    dp[0][n - 1]
}

fn main() {
    let arr = vec![1, 2, 3, 4, 3];
    let result = matrix_multiplication(&arr);

    println!("Minimum number of multiplications is: {}", result);
}
