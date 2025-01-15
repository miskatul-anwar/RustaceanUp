use std::io::{stdin, BufRead};

fn _rin() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().chars().collect()
}

fn pattern_matching(t: Vec<char>, p: Vec<char>) -> usize {
    let r = p.len();
    let s = t.len();
    let mut index = 0;

    let max = s - r + 1;

    let mut k = 0;
    while k < max {
        for l in 0..r {
            if p[l] != t[k + l] {
                index = k;
                break;
            }
        }
        k += 1;
    }
    index
}

fn main() {
    let t = _rin();
    let p = _rin();
    let index = pattern_matching(t, p);
    println!("Position: {}", index);
}
