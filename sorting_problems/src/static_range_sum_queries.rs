/*
This template is made by Miskatul Anwar<miskatul.anwar.csecu@gmail.com>
GitHub : https://github.com/miskatul-anwar

Thank You
*/

#![allow(unused)]

use std::io::stdin;

fn prefix_sum(array: &Vec<usize>) -> Vec<usize> {
    let mut prefix_array = vec![0; array.len() + 1];
    for i in 1..=array.len() {
        prefix_array[i] = prefix_array[i - 1] + array[i - 1];
    }
    prefix_array
}

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn main() {
    let n_q = rin_vec_int();
    let x = rin_vec_int();

    let pref = prefix_sum(&x);

    for _ in 0..n_q[1] {
        let a_b = rin_vec_int();
        let left = a_b[0];
        let right = a_b[1];
        // Using prefix sum to compute the range sum
        let ans = pref[right] - pref[left - 1];
        println!("{ans}");
    }
}
