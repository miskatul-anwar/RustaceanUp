/*
This template is made by Miskatul Anwar<miskatul.anwar.csecu@gmail.com>
GitHub : https://github.com/miskatul-anwar

Thank You
*/

#![allow(unused)]

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

fn rin_vec_char() -> Vec<char> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    return vec;
}

fn to_string(vec: Vec<char>) -> String {
    return vec.iter().collect::<String>();
}

pub fn main() {
    let mut n = rin_int();
    print!("{n} ");
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            print!("{n} ");
        } else {
            n *= 3;
            n += 1;
            print!("{n} ");
        }
    }
    println!();
}
