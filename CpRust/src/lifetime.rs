use text_io::read;

fn sl<'a>(s: &'a str) -> &'a str {
    &s[0..=1]
}

fn main() {
    let name: String = read!();
    println!("{}", sl(&name));
}
