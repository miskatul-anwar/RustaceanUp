use std::io::{stdin, BufRead};

fn rin() -> i32 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let t: i32 = rin();

    for _ in 1..=t {
        let a: i32 = rin();
        let b: i32 = rin();
        let xk: i32 = rin();
        let yk: i32 = rin();
        let xq: i32 = rin();
        let yq: i32 = rin();
    }
}
