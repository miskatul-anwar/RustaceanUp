use std::collections::HashMap;

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";

    let mut freq: HashMap<&str, i32> = HashMap::new();

    for i in text.split_whitespace() {
        *freq.entry(i).or_insert(0) += 1;
    }

    for i in &freq {
        println!("{} => {}", i.0, i.1);
    }
}
