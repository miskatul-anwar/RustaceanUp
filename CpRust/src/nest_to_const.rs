use std::io::{stdin, BufRead};

fn _rin() -> i32 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let n = _rin();
    let m = _rin();
    let mut matrix = vec![vec![0; m as usize]; n as usize];

    let mut i = 0;
    let mut j = 0;

    while i < n {
        matrix[i as usize][j as usize] = i + j;
        j += 1;
        if j == m {
            j = 0;
            i += 1;
        }
    }

    for row in matrix {
        println!("{row:?}");
    }
}

