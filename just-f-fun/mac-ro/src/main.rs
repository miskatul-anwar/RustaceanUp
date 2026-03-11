macro_rules! math {
    (add $x:expr, $y:expr) => {
        $x + $y
    };
    (mul $x:expr,$y:expr) => {
        $x * $y
    };
}
fn main() {
    println!("Sum of 5 and 10 is: {}", math!(add 5, 10));
    println!("Product of 5 and 10 is: {}", math!(mul 5, 10));
}
