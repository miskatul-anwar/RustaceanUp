/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

use std::io::{stdin, BufRead};

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn main() {
    let mut line = rin_vec_int();
    line.sort_unstable();

    let ans = (line[1] - line[0]) + (line[2] - line[1]);
    println!("{}", ans);
}
