/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

use std::io::{stdin, BufRead};

fn rin_i32() -> i32 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_i32() -> Vec<i32> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solution() {
    let n: usize = rin_i32() as usize;
    let k: usize = rin_i32() as usize;

    let ar = rin_vec_i32();

    if n == k {
        let mut ans = -1;
        for i in (2..=n).step_by(2) {
            if ar[i - 1] != (i / 2) as i32 {
                ans = (i / 2) as i32;
                break;
            }
        }
        if ans == -1 {
            ans = (n / 2 + 1) as i32;
        }
        println!("{}", ans);
    } else {
        let mut posi = -1;
        for i in 2..=(n - k + 2) {
            if ar[i - 1] != 1 {
                posi = i as i32;
                break;
            }
        }
        if posi == -1 {
            let mut curr = 2;
            for i in (n - k + 1..=n).step_by(2) {
                if ar[i - 1] != curr {
                    break;
                }
                curr += 1;
            }
            println!("{}", curr);
        } else {
            println!("1");
        }
    }
}

fn main() {
    let t = rin_i32();
    for _ in 0..t {
        solution();
    }
}
