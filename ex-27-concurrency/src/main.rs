use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..5 {
            println!("in thread [1]: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("in main: {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("in thread [2]: {}", i);
            thread::sleep(Duration::from_millis(50));
        }
    });

    with_move();

    handle.join().unwrap();
}

fn with_move() {
    let x = 100;

    thread::spawn(move || {
        println!("x = {}", x);
    });
}

// --> src/main.rs:32:19
// |
// 32 |     thread::spawn(|| {
// |                   ^^ may outlive borrowed value `x`
// 33 |         println!("x = {}", x);
// |                            - `x` is borrowed here
// |
// note: function requires argument type to outlive `'static`
// --> src/main.rs:32:5
// |
// 32 | /     thread::spawn(|| {
// 33 | |         println!("x = {}", x);
// 34 | |     });
// | |______^
// help: to force the closure to take ownership of `x` (and any other referenced variables), use the `move` keyword
// |
// 32 |     thread::spawn(move || {
// |                   ++++
