fn main() {
    let v: Vec<i32> = (1..=10).collect();
    let v_sq: Vec<i32> = v.iter().map(|&i| i * i).collect();
    for i in v_sq {
        print!("{} ", i);
    }
    println!();
}
