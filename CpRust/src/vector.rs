struct HashSet<T: PartialEq + Clone> {
    data: Vec<T>,
    size: usize,
}

impl<T: PartialEq + Clone> HashSet<T> {
    fn new() -> Self {
        HashSet {
            data: Vec::new(),
            size: 0,
        }
    }

    fn insert(&mut self, value: T) {
        let mut found = false;
        for i in &self.data {
            if *i == value {
                found = true;
                break;
            }
        }
        if !found {
            self.data.push(value);
            self.size += 1;
        }
    }

    fn values(&self) -> &[T] {
        &self.data
    }
}

fn main() {
    let mut my_hashset = HashSet::new();

    // Insert elements
    my_hashset.insert(10);
    my_hashset.insert(20);
    my_hashset.insert(30);
    my_hashset.insert(20); // Duplicate, won't be added

    // Show elements
    for element in my_hashset.values() {
        println!("{}", element);
    }
}
