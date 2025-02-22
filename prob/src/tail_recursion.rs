#[inline]
fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn main() {
    for i in 0..=8 {
        print!("{} ", fibonacci(i))
    }
    println!()
}
