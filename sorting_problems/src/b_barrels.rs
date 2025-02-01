#![allow(unused)]
use std::io::{self, BufRead};

const MXN: usize = 2 * 1e5 as usize; // Set constant for the maximum size of the array

fn rin_int() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    // Read n and k
    let n_k = rin_vec_int();
    let n = n_k[0];
    let mut k = n_k[1];

    // Read array a
    let mut a = rin_vec_int();

    // Sort array a
    a.sort();

    // Get max from the last element after sorting
    let mut max = a[n - 1];

    // Add elements from the rest of the array to max while k is not 0
    for i in (0..n - 1).rev() {
        if k != 0 {
            max += a[i];
            a[i] = 0; // Set to 0, although unnecessary for the result
            k -= 1;
        }
    }

    // Get the minimum value in the array after modifications
    let min = *a.iter().min().unwrap();

    // Output the result
    println!("{}", max - min);
}

fn main() {
    // Input reading with multiple test cases
    let t = rin_int();
    for _ in 0..t {
        solve();
    }
}
