fn fibonacci(n: i128) -> i128 {
    let (mut prev_prev, mut prev, mut curr) = (0, 1, 1);

    for _ in 2..=n {
        curr = prev + prev_prev;
        prev_prev = prev;
        prev = curr;
    }

    curr
}

fn main() {
    let n = 6;

    println!("Fibonacci of {} is: {}", n, fibonacci(n))
}
