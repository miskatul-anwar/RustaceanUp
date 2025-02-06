use std::io::{stdin, BufRead};

fn main() {
    let mut input: String = String::new();
    stdin().lock().read_line(&mut input).unwrap();

    let t: i8 = input.trim().parse().unwrap();

    for _ in 1..=t {
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        let n: usize = input.trim().parse().unwrap();

        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        let v: Vec<i32> = input
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();

        if v.windows(2).all(|w| w[0] <= w[1]) {
            let mut min_diff = i32::MAX;

            for i in 0..n - 1 {
                min_diff = min_diff.min(v[i + 1] - v[i]);
            }

            println!("{}", min_diff / 2 + 1);
        } else {
            println!("0");
        }
    }
}
