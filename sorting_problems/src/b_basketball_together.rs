/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#![allow(unused)]

use std::{
    collections::VecDeque,
    io::{stdin, BufRead},
};

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let (n_d, mut p) = (rin_vec_int(), rin_vec_int());
    let (n, d) = (n_d[0], n_d[1]);

    p.sort_unstable_by(|a, b| b.cmp(a));

    let (mut player, mut i, mut wins) = (0, 0, 0);

    while i < n && player <= n {
        let x = ((d + 1) as f64 / (p[i] as f64 * 1.0)).ceil();
        player += x as usize;

        if player <= n {
            wins += 1
        }

        i += 1
    }

    println!("{}", wins)
}
