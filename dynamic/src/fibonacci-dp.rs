const N: usize = 100;

fn fib(n: usize, dp: &mut [i32]) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            if dp[n] != -1 {
                return dp[n];
            }

            dp[n] = fib(n - 1, dp) + fib(n - 2, dp);
            dp[n]
        }
    }
}

fn fib2(n: usize, dp: &mut [i32]) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            for i in 2..=n {
                dp[i] = dp[i - 1] + dp[i - 2]
            }

            dp[n]
        }
    }
}

fn main() {
    let mut dp: [i32; N] = [-1; N];
    let n = 9;

    // Top Down Approach
    println!("Fibonacci of {}: {}", n, fib(n, &mut dp));

    let mut dp: [i32; N] = [-1; N];
    (dp[0], dp[1]) = (0, 1);

    // Bottom Up Approach
    println!("Fibonacci of {}: {}", n, fib2(n, &mut dp))
}
