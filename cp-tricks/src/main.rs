use num::Complex;

fn main() {
    println!("Hello, world!");
    let a = Complex { re: 2.1, im: -1.2 };
    println!("{} {}", a.re, a.im);
    let mut arr = [0, 1, 2, 3, 4];

    for i in &mut arr {
        *i += 1
    }

    for i in arr {
        print!("{} ", i)
    }

    println!()
}
