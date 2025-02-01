use std::io::{stdin, BufRead};
fn rin_i16() -> i16 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_i8() -> i8 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn rin_vec_char() -> Vec<char> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.chars().collect()
}

fn main() {
    let t = rin_i16();

    for _ in 1..=t {
        let _n = rin_i8();
        let s = rin_vec_char();
        let max = (*s.iter().max().unwrap() as i32) - 96;

        println!("{max}");
    }
}
