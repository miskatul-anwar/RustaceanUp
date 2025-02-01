/*
This template is made by Miskatul Anwar<miskatul.anwar.csecu@gmail.com>
GitHub : https://github.com/miskatul-anwar

Thank You
*/

#![allow(unused)]

use std::io::{stdin, BufRead};

fn rin_vec_char() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let vec: Vec<char> = input.trim().chars().collect();
    return vec;
}

pub fn main() {
    let input = rin_vec_char();

    let mut check_rep = 1;
    let mut max_rep = 1;

    for i in 1..input.len() {
        if input[i] == input[i - 1] {
            check_rep += 1;
        } else {
            max_rep = max_rep.max(check_rep);
            check_rep = 1;
        }
    }
    max_rep = max_rep.max(check_rep);
    println!("{max_rep}");
}
