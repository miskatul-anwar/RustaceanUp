fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(2);
    v.push(1);
    v.push(9);
    v.push(3);
    for i in v.iter().rev() {
        print!("{i:?} ");
    }
}
