use std::collections::LinkedList;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(2);
    list.push_front(9);
    for i in &list {
        print!("{}", &i);
    }
    list.clear();
}
