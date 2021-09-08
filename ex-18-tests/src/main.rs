mod arithmetic;
mod powers;

fn main() {
    let num1 = 62;
    let num2 = 13;

    println!("is odd: {} -> {}", num1, arithmetic::is_odd(&num1));
    println!("is even: {} -> {}", num2, arithmetic::is_odd(&num2));

    println!("square: {} -> {}", num2, powers::square(num2));
    println!("cube: {} -> {}", num1, powers::cube(num1));

    println!("49/8 = {}", arithmetic::div(49, 8));
}
