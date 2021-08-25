fn main() {
    // number types
    let n1: i8 = 127;
    let n2: i16 = 30_239;
    let n3: i32 = 127_213_123;
    // isize and usize types depend on the kind of computer your program is running on:
    // 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    let arch: isize = 127;

    println!("n1: i8 = {}", n1);
    println!("n2: i16 = {}", n2);
    println!("n3: i32 = {}", n3);
    println!("arch: isize = {}", arch);


    println!("div: {}", n2 as f64 / n1 as f64);

    let tup = (12, 67, 8.99);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", x, y, z);

    let tup2: (char, i32, f64) = ('Z', 78, 9.99);
    println!("tup2 = {:?}", tup2);
    println!("tup2.0 = {}", tup2.0);
    println!("tup2.1 = {}", tup2.1);
    println!("tup2.2 = {}", tup2.2);

    let a1: [i32; 5] = [10, 23, 28, 89, 99_999];
    println!("a1: {:?}", a1);
    // index out of bounds: the length is 5 but the index is 100
    // println!("a1: {}", a1[100]);
    println!("a1: {}", a1[4]);
}
