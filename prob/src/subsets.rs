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

/*~~~~~~~~~~~~~*
 * CODE BELOW: *
 *~~~~~~~~~~~~~*/

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut subset = vec![];
    backtrack(&nums, 0, &mut subset, &mut result);
    result
}

fn backtrack(nums: &Vec<i32>, index: usize, subset: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    result.push(subset.clone()); // Store the current subset

    for i in index..nums.len() {
        subset.push(nums[i]); // Include nums[i]
        backtrack(nums, i + 1, subset, result);
        subset.pop(); // Backtrack (remove last added element)
    }
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let t = sc.next();
    for _ in 1..=t {
        let mut n = sc.next();
        let nums = (0..n).map(|_| sc.next()).collect();

        println!("{:?}", subsets(nums));
    }
}
