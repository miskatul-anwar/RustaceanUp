type int = i64;
fn main() {
    let msg = "Tutorials Point has good t
   utorials"
        .to_string();
    let mut i: int = 1;

    for token in msg.split_whitespace() {
        println!("token {} {}", i, token);
        i += 1;
    }
}
