use std::collections::HashMap;

fn main() {
    let mut hashmap = HashMap::new();
    println!("Coding Expertise... ");
    hashmap.insert(String::from("Miskat"), -666);
    hashmap.insert(String::from("Debashish"), 75);
    hashmap.insert(String::from("Emon"), 99);
    hashmap.insert(String::from("Sahil"), 50);

    for i in hashmap.iter() {
        println!("{} {}", i.0, i.1)
    }
}
