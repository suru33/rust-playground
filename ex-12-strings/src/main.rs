fn main() {
    let s1 = String::from("String 1");
    println!("s1 = {}", s1);

    let s2 = "String 2".to_string();
    println!("s2 = {}", s2);

    let s3 = String::from("Здравствуйте"); //UTF-8 String
    println!("s3 = {}", s3);

    let mut s4 = String::from("Hello ");
    println!("s4 = {}", s4);
    s4.push_str("World");
    println!("s4 = {}", s4);
    s4.push('!');
    println!("s4 = {}", s4);
    println!("len({}) = {}", s4, s4.len());

    let s5 = String::from("Kill");
    let s6 = String::from("Bill");
    let s7 = s5 + " " + &s6;
    // println!("s5 = {}", s5);
    // can not use `s5` anymore!
    // -------- `s5` moved due to usage in operator
    println!("s7 = {}", s7);

    let s8 = "tic".to_string();
    let s9 = "tac".to_string();
    let s10 = "toe".to_string();

    let s11 = format!("{}-{}-{}", s8, s9, s10);
    println!("s11 = {}", s11);
    // can use variables after `format!`
    println!("s9 = {}", s8);
    println!("s9 = {}", s9);
    println!("s10 = {}", s10);

    // slicing
    println!("s3 = {}", s3);
    let s12 = &s3[0..4]; // slice by bytes -> UTF takes 2bytes for each chars
    println!("s12 = {}", s12);
    // can not slice here! the char size is 2 bytes
    // let s13 = &s3[0..1];
    // println!("s13 = {}", s13);
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'

    println!("Chars in {}", s3);
    for ch in s3.chars() {
        println!("{}", ch);
    }

    println!("Bytes in {}", s3);
    for ch in s3.bytes() {
        println!("{}", ch);
    }
}
