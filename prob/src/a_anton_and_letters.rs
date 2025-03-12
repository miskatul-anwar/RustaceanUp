use std::{
    collections::HashSet,
    io::{stdin, stdout, BufWriter, Write},
};

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let out = &mut BufWriter::new(stdout());

    let s = read_line();
    let mut set = HashSet::new();

    for ch in s.chars() {
        if ch >= 'a' && ch <= 'z' {
            set.insert(ch);
        }
    }

    writeln!(out, "{}", set.len()).unwrap()
}
