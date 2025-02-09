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
    /* FIXME: */
    let (n_d, mut p) = (rin_vec_int(), rin_vec_int());
    let (n, d) = (n_d[0], n_d[1]);

    p.sort_unstable_by(|a, b| b.cmp(a));
}
