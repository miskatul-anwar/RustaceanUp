fn main() {
    let x = Some(1);

    match x {
        Some(1) => println!("Either 1"),
        _ => println!("Other value"),
    }
}
