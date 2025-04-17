#![allow(unused)]
use std::io::{stdin, stdout, BufWriter, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn count_less_or_equal(sorted_arr: &[i64], value: i64) -> usize {
    let mut low = 0;
    let mut high = sorted_arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if sorted_arr[mid] <= value {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let test_cases: i32 = sc.next();
    for _ in 0..test_cases {
        let array_size: usize = sc.next();
        let copy_count: i32 = sc.next();
        let target_value: i64 = sc.next();

        let mut elements = vec![0; array_size];
        for i in 0..array_size {
            elements[i] = sc.next();
        }

        let mut prefix_sums = vec![0; array_size];
        for i in 1..array_size {
            prefix_sums[i] = prefix_sums[i - 1] + elements[i - 1];
        }
        let total_sum = prefix_sums[array_size - 1] + elements[array_size - 1];

        if (copy_count as i64) * total_sum < target_value {
            writeln!(out, "0").unwrap();
            continue;
        }

        let remaining_sum = (copy_count as i64) * total_sum - target_value;
        let mut total_count = 0;

        for current_copy in 0..copy_count {
            let limit = remaining_sum - (current_copy as i64) * total_sum;
            if limit < 0 {
                continue;
            }
            let count = count_less_or_equal(&prefix_sums, limit);
            total_count += count;
        }

        writeln!(out, "{}", total_count).unwrap();
    }
}
