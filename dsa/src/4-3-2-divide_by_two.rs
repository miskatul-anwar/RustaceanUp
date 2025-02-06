use text_io::read;
fn base_converter(mut dec: i32, base: i32) -> String {
    let mut rem_stack = Vec::new();
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    while dec > 0 {
        rem_stack.push(dec % base);
        dec /= base;
    }
    
    let mut base_str = String::new();

    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
    
}

fn main() {
    let dec = read!();
    let base = read!();
    let bin: String = base_converter(dec, base);
    println!("{bin}");
}