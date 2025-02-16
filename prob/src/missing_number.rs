/*
This template is made by Miskatul Anwar<miskatul.anwar.csecu@gmail.com>
GitHub : https://github.com/miskatul-anwar

Thank You
*/

use std::io::stdin;

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

pub fn main() {
    let n = rin_int();
    let nums = rin_vec_int();
    let sum: usize = nums.iter().sum();

    let mut ac_sum = 0;

    for i in 1..=n {
        ac_sum += i;
    }

    println!("{}", ac_sum - sum);
}
