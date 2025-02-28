#![allow(unused)]
use core::num;
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

fn left_max(nums: &Vec<i32>, ans: &mut Vec<i32>, pos: usize, mut greatest: i32) {
    if pos == nums.len() {
        return;
    }

    if greatest < nums[pos] {
        greatest = nums[pos];
    }

    ans.push(greatest);
    left_max(nums, ans, pos + 1, greatest);
}

fn main() {
    let mut sc = Scanner::default();
    let out = &mut BufWriter::new(stdout());

    let n = sc.next();
    let nums: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let mut ans: Vec<i32> = Vec::new();
    let mut greatest = nums[0];

    left_max(&nums, &mut ans, 0, greatest);

    for i in ans {
        write!(out, "{} ", i).unwrap()
    }
}
