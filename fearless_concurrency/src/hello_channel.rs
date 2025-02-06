use std::{
    sync::mpsc::channel,
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let (tx, rx) = channel();

    let msg = "Hello, World!";

    let handle1 = thread::spawn(move || {
        println!("Sent: ");
        let _ = tx.send(msg);
        sleep(Duration::from_millis(1));
    });

    handle1.join().unwrap();

    let handle2 = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("{}", received);
        sleep(Duration::from_millis(1));
    });

    handle2.join().unwrap();
}
