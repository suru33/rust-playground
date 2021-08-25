fn main() {
    let s1 = String::from("Hello World");
    println!("s1: {}", s1);

    let s2 = s1;
    // println!("s1: {}", s1);
    // doesn't work, value already moved to s2
    println!("s2: {}", s2);

    let s3 = String::from("Random String");
    let (s4, len) = calculate_length(s3);
    // println!("s3: {}", s3);
    // doesn't work, value moved to calculate_length
    // and then returned to s4
    println!("s4: {}, len: {}", s4, len);

    // References and Borrowing
    let ref_len = calculate_ref_length(&s4);
    // only reference is passed to function
    // s4 is still accessible here
    // but you can't modify value
    println!("s4: {}, ref_len: {}", s4, ref_len);

    let mut s5 = String::from("Hello");
    println!("s5: {}", s5);
    change(&mut s5);
    println!("s5: {}", s5);

    let s6 = "Hello str";
    // not possible to use more than one `mut reference`
    // let s6_ref_1 = &mut s6;
    // let s6_ref_2 = &mut s6;
    //
    // println!("s6_ref_1: {}", s6_ref_1);
    // println!("s6_ref_2: {}", s6_ref_2);
    println!("s6: {}", s6);
    println!("s6: {}", s6);

    // The Rules of References
    //     At any given time, you can have either one mutable reference or any number of immutable references.
    //     References must always be valid.
}

fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}

fn calculate_ref_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World");
}