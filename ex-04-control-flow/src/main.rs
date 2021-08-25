fn main() {
    let x = 10;
    if x < 2 {
        println!("true");
    } else {
        println!("false")
    }

    if x <= 10 {
        println!("hello world");
    } else if x > 10 {
        println!("hello world 1");
    } else {
        println!("ELSE");
    }

    // as expression
    let y = 72.3;
    let grade = if y > 75.0 {
        "A"
    } else if y > 65.0 {
        "B"
    } else if y > 55.0 {
        "C"
    } else {
        "D"
    };
    println!("Grade: {}", grade);

    // infinite-loop
    // loop {
    //     println!("***");
    // }

    // loop as expression
    let mut counter = 0;

    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop_result: {}", loop_result);

    while counter < 50 {
        counter += 1;
    }
    println!("New counter: {}", counter);

    // for

    let arr = [10, 20, 30, 40, 50];

    for i in arr.iter() {
        println!("i = {}", i);
    }

    for (ix, element) in arr.iter().enumerate() {
        println!("ix = {}, element: {}", ix, element);
    }

    // range
    for i in 100..105 {
        println!("i = {}", i)
    }

    for i in (100..105).rev() {
        println!("i = {}", i)
    }
}
