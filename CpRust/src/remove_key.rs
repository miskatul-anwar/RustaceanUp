use std::collections::HashMap;

fn main() {
    let mut state_codes: HashMap<i32, &str> = HashMap::new();
    state_codes.insert(1, "apple");
    state_codes.insert(3, "pine");
    state_codes.insert(2, "orange");
    state_codes.insert(4, "banana");
    state_codes.remove(&3);
    for i in state_codes {
        println!("{}", i.1);
    }
}
