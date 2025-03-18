// Use the iterative approach to
// calculate the nth Catalan number

fn catalan(n: usize) -> usize {
    let mut res = 1;

    for i in 2..=n {
        res = (res * (4 * i - 2)) / (i + 1)
    }

    res
}

fn main() {
    let n = 6;
    println!("{}", catalan(n));
}
