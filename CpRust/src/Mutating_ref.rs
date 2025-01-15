fn increment(i: &mut i32) {
    *i += 1;
}

fn display(param: &mut String) {
    param.push_str("Fku");
}

fn slice(param: &mut String) {
    *param = param[1..4].to_string();
}

fn main() {
    let mut i = 0;
    increment(&mut i);
    println!("{}", i);
    let mut str: String = String::from("Hello World!");
    display(&mut str);
    println!("{}", str);
    slice(&mut str);
    println!("{str}");
}
