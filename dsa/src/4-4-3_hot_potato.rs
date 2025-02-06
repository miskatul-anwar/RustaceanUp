use std::collections::VecDeque;
/* Josephus Problem */
fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = VecDeque::new();
    for name in names {
        q.push_back(name); // enqueue
    }

    while q.len() > 1 {
        for _ in 1..=num {
            if let Some(deq) = q.pop_front() {
                q.push_back(deq);
            }
        }

        q.pop_front(); // dequeue
    }

    let mut survivor = "";
    if let Some(deq) = q.pop_front() {
        survivor = deq;
    }

    &survivor
}

fn main() {
    let names = vec!["Mon", "Tom", "Kew", "Lisa", "Marry", "Bob"];
    let survivor = hot_potato(names, 8);
    println!("The survival person is {survivor}");
}
