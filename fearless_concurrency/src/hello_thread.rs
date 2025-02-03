use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Hi, number {i} from the spawned thread!");
        }
    });

    for i in 1..=5 {
        println!("Hi, number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
