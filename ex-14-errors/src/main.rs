use std::fs::File;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    // simple instant crash
    // panic!("Crash......"); // exit code 101

    let v = vec![100, 12, 43, 13];
    // v[10]; // thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 10'
    println!("vec = {:?}", v);

    let f = File::open("hello.txt");

    println!("file f = {:?}", f);
    // file f = Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })

    // let f = match f {
    //     Ok(f) => f,
    //     Err(e) => panic!("File not found: {:?}", e),
    // };

    let f = File::open("test.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(file) => file,
                Err(e) => panic!("Failed to create file: {:?}", e),
            }
            other_error => panic!("Error accessing file: {:?}", other_error),
        }
    };
    println!("File: {:?}", f);

    // another way
    let f = File::open("test-2.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("test-2.txt").unwrap_or_else(|e| {
                panic!("failed to create file: {:?}", e)
            })
        } else {
            panic!("failed to open file: {:?}", e)
        }
    });

    println!("f = {:?}", f);
}
