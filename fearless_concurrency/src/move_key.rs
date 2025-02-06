use std::{thread, time::Duration};

fn main() {
    let question = String::from("Answer 02");

    let handle = thread::spawn(move || {
        println!("{question}");
        for i in (2..=8).step_by(2) {
            println!("{i} from thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in (1..8).step_by(2) {
        println!("{i} from main");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
