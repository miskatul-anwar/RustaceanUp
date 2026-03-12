fn main() {
    let s1 = "abcd";
    let s2 = "abcd";
    let res = longest(s1, s2);
    println!("{}", res)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    } else {
        y
    }
}
