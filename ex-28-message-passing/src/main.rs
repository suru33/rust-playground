use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("message");
        tx.send(msg).unwrap();
        // `msg` is already moved you can not use anymore
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
            String::from("lang"),
        ];

        for msg in messages {
            tx2.send(msg).unwrap();
        }
    });

    for msg in rx2 {
        println!("message from rx2: {}", msg);
    }
}
