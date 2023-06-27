use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    thread::spawn(move || {
        for i in 1..4 {
            tx.send(format!("from [tx]: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        for i in 1..7 {
            tx2.send(format!("from [tx2]: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        for i in 1..4 {
            tx3.send(format!("from [tx3]: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    for msg in rx {
        println!("{}", msg)
    }
}
