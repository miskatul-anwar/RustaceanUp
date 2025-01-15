use std::io::{stdin, BufRead};

fn main() {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let t: i16 = input.trim().parse().unwrap();

    for _ in 1..=t {
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        let n: usize = input.trim().parse().unwrap();

        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        let mut a: Vec<i32> = input
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        for i in 0..a.len() {
            if a[i] == 1 {
                a[i] += 1
            }
        }

        for i in 0..n - 1 {
            if a[i + 1] % a[i] == 0 {
                a[i + 1] += 1
            }
        }

        for i in a {
            print!("{i} ")
        }
        println!();
    }
}
