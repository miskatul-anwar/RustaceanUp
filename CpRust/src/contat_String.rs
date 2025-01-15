fn main() {
    let s1 = String::from("Hello, World! ");
    let s2 = String::from("Miskat.");
    let mut s3 = String::new();
    s3 += &s1;
    s3 += &s2;

    let output = format!("{s1} + {s2} = {s3}\n");

    print!("{output}");
}
