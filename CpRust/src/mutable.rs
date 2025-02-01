use std::io::stdin;

fn _rin() -> String {
    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let t: i32 = 90;
    println!("{t:?}");
}
