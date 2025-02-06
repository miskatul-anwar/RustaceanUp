/*
This template is made by Miskatul Anwar<miskatul.anwar.csecu@gmail.com>
GitHub : https://github.com/miskatul-anwar

Thank You
*/

use std::io::stdin;

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn main() {
    let mut n = rin_int();
    print!("{n} ");
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            print!("{n} ");
        } else {
            n *= 3;
            n += 1;
            print!("{n} ");
        }
    }
    println!();
}
