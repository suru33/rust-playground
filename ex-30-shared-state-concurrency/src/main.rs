use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// https://doc.rust-lang.org/book/ch16-03-shared-state.html

fn main() {
    let m = Mutex::new(100);

    {
        let mut num = m.lock().unwrap();
        *num = 22;
    }

    println!("m = {:?}", m);

    // Arc: Atomic Reference Counting
    // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations

    let counter = Arc::new(Mutex::new(0));
    let mut counter_threads = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        counter_threads.push(handle);
    }

    for th in counter_threads {
        th.join().unwrap();
    }

    println!("counter = {:?}", counter);
}
