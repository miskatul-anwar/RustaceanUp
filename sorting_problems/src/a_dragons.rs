/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

#[allow(unused)]
use std::io::{stdin, BufRead};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    x: usize,
    y: usize,
}

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
    let line1 = rin_vec_int();
    let (mut s, n) = (line1[0], line1[1]);

    let mut xy: Vec<Pair> = Vec::new();

    for _ in 1..=n {
        let line = rin_vec_int();
        xy.push(Pair {
            x: line[0],
            y: line[1],
        });
    }

    xy.sort_unstable();

    let mut possible = true;

    for pair in xy {
        if s > pair.x {
            s += pair.y;
        } else {
            possible = false;
            break;
        }
    }

    if possible {
        println!("YES");
    } else {
        println!("NO");
    }
}
