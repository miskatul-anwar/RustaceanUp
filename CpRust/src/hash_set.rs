use std::collections::{BTreeSet, HashSet};
type Hsi32 = HashSet<i32>;
type Bsi32 = BTreeSet<i32>;
fn main() {
    let mut set: Hsi32 = Hsi32::new();
    let mut btset: Bsi32 = Bsi32::new();

    // for unordered set
    set.insert(3);
    set.insert(1);
    set.insert(9);

    for i in &set {
        print!("{} ", i);
    }

    //for ordered set
    btset.insert(3);
    btset.insert(8);
    btset.insert(1);
    println!();
    for i in &btset {
        print!("{} ", i);
    }
}
