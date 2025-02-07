/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

use std::io::{stdin, BufRead};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    /* *************************(  Code Here  )************************* */
    let (n, s) = (rin_int(), rin_vec_int());

    let mut ss = s.clone();
    ss.sort_by(|a, b| b.cmp(a)); //sort reverse

    for i in 0..n {
        if s[i] == ss[0] {
            // if it's the max element,
            // subtract the next second max element...
            print!("{} ", s[i] - ss[1]);
            continue;
        }
        // else, all you need to do is to just subtract
        // the max element from others
        print!("{} ", s[i] - ss[0]);
    }
    println!()
}

pub fn main() {
    let t = rin_int();
    for _ in 1..=t {
        solve();
    }
}
