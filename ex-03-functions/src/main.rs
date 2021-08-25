fn main() {
    println!("Hello, world!");
    another_function();
    print_sum(100, 2);

    // function block or expression
    let x = {
        let y = 100;
        y + 23
    };

    println!("x = {}", x);

    let (a, b) = (10, 2);
    println!("a + b = {}", sum(a, b));
}

fn another_function() {
    println!("Hello another function");
}

fn print_sum(a: i32, b: i32) {
    println!("Sum: {} + {} = {}", a, b, a + b);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b // or return a + b;
}