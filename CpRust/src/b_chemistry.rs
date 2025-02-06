use std::io::{self, BufRead};

const B: usize = 26;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let t: usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let line1: Vec<i64> = input
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let _n = line1[0];
        let k = line1[1];

        let s: Vec<char> = input.next().unwrap().unwrap().chars().collect();
        let mut v = vec![0; B];
        for ch in &s {
            v[*ch as usize - 'a' as usize] += 1;
        }

        let mut cnt = 0;
        for p in 0..B {
            cnt += v[p] % 2;
        }

        if cnt <= (k + 1) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
