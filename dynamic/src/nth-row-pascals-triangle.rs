/*
 nCi = n! / (i! * (n-i)!)   – ith element of nth row
 nCi+1 = n! / ((i+1)! * (n-(i+1))! )  – (i+1)th element of nth row
 Dividing  (i+1)th element by ith element will give:
 nCi+1 / nCi = i! * (n-i)! / (i+1)! * (n-i-1)!  – On solving this
 nCi+1 = nCi  * (n-i) / (i+1) –  we get (i+1)th element with the help of ith element
*/

fn nth_row_pascals(n: usize) -> Vec<usize> {
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        dp[i] = dp[i - 1] * (n - i + 1) / i;
    }

    dp
}

fn main() {
    let nth: usize = 5;
    let nth_row = nth_row_pascals(nth);

    for i in nth_row {
        print!("{} ", i);
    }
}
