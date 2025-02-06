use std::io::{stdin, BufRead};
fn rin_i16() -> i16 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}
fn main() {
    let _n = rin_i16();
}
