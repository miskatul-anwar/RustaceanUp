use std::collections::HashMap;

fn main() {
    let mut array: HashMap<i32, i32> = HashMap::new();

    array.insert(0, 5);
    array.insert(1, 2);
    array.insert(2, 3);
    array.insert(3, 1);

    for i in 0..4 {
        println!("{}", array[&i])
    }
}
