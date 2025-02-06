use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<char, i32> = HashMap::new();
    let str: String = "she sells sea shells in the sea shore".to_string();

    for ch in str.chars().filter(|i| !i.is_whitespace()) {
        *hm.entry(ch).or_insert(0) += 1;
    }

    for (key, value) in &hm {
        println!("{key:?} => {value:?}");
    }
}

