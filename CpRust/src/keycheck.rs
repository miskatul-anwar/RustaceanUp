use std::collections::HashMap;

fn main() {
    let mut state_codes = HashMap::new();
    state_codes.insert("KL", "Kerala");
    state_codes.insert("MH", "Maharashtra");
    state_codes.insert("GJ", "Gujarat");
    if state_codes.contains_key(&"GJ") {
        println!("Yes");
    }
}
