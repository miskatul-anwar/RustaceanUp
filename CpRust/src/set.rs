use std::io::{stdin, BufRead};

struct HashSet {
    data: Vec<i32>,
    size: usize,
}

impl HashSet {
    fn insert(&mut self, val: i32) {
        let mut found: bool = false;
        for i in &self.data {
            if *i == val {
                found = true;
                break;
            }
        }
        if !found {
            self.data.push(val);
            self.size += 1;
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn display(&self) {
        for i in &self.data {
            print!("{i} ")
        }
    }
}

fn _rin() -> i32 {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut hashset = HashSet {
        data: Vec::new(),
        size: 0,
    };

    let size = _rin();
    for _ in 0..size {
        hashset.insert(_rin());
    }
    println!("The size of the hashset: {}", hashset.len());
    hashset.display();
}
