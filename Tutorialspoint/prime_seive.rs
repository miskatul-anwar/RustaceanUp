use std::io::{self, BufRead};

fn prime_sieve(n: i64) -> Vec<i64> {
    let mut prime_numbers = Vec::new();
    let mut mark_prime = vec![true; (n + 1) as usize];
    let p: i64 = 2;

    for i in (p * p..=n).step_by(p as usize) {
        mark_prime[i as usize] = false;
    }

    for i in 2..=n {
        if mark_prime[i as usize] {
            prime_numbers.push(i);
        }
    }

    prime_numbers
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();

    let n: i64 = input.trim().parse().unwrap();

    for prime in prime_sieve(n) {
        print!("{} ", prime);
    }
    println!();
}
