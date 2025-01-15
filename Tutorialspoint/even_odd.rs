use std::io::{stdin, BufRead};
fn main() {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    let t: i64 = input.trim().parse().unwrap();
    for _ in 1..=t {
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();
        let n: i64 = input.trim().parse().unwrap();
        if n % 2 == 0 && n > 1 {
            print!("{} is an even number!", n);
        } else {
            print!("{} is an odd number!", n);
        }
    }
}
