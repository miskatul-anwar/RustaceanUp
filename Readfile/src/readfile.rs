use std::fs::read_to_string;

fn main() {
    println!("File Reading program!");
    let contents = read_to_string("./data.txt").expect("Maybe There's no file named `data.txt`");
    println!("From the file: {contents}");
}
