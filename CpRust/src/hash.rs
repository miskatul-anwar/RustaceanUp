use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn main() {
    let mut hm: HashMap<i32, String> = HashMap::new();
    hm.insert(1, "Miskatul Anwar".to_string());
    hm.insert(3, "Debashish Chakraborty".to_string());
    hm.insert(8, "Abdullah Al Mamun Emon".to_string());
    print!("Select Option:\n1.View Keys\n2.View Values\n=> \n");
    let mut input: String = String::new();
    stdin()
        .lock()
        .read_line(&mut input)
        .expect("NO Values were passed!");
    let option: i32 = input.trim().parse().unwrap();
    input.clear();
    if option == 1 {
        for i in hm {
            println!("{}", i.0);
        }
    } else if option == 2 {
        for i in hm {
            println!("{}", i.1);
        }
    } else {
        println!("Invalid Selection!");
    }
}
