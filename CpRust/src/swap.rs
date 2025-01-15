use std::mem::replace;

fn main() {
    let mut x = 6;
    let mut y = 5;
    x = replace(&mut y, x);
    println!("{x} {y}");
}
