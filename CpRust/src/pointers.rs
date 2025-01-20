fn main() {
    let mut my_num = 14;
    let x = &my_num;
    println!("{x}");
    my_num += 1;
    let x = &my_num;
    println!("{x}");
}
