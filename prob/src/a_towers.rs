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

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    return arr;
}

fn solve() {
    /* *************************(  Code Here  )************************* */
    let (_n, l, mut num, mut count, mut size) = (rin_int(), rin_vec_int(), 0, vec![0; 1002], 0);

    for i in l {
        count[i] += 1;
    }

    for &i in count.iter() {
        if i != 0 {
            num += 1
        }
        if size < i {
            size = i;
        }
    }

    println!("{} {}", size, num);
}

pub fn main() {
    solve()
}
