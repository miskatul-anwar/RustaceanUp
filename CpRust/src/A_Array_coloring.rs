use std::io::{stdin, BufRead};

fn main() {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();

    let t: i16 = input.trim().parse().unwrap();

    for _ in 1..=t {
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        let _n: i8 = input.trim().parse().unwrap();

        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        let v: Vec<i8> = input
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        input.clear();

        let sum: i32 = v.iter().map(|&i| i as i32).sum();

        if sum % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

