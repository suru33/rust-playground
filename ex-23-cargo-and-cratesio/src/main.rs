use crates_io_test::simple_math;

fn main() {
    let a: f64 = 345.0;
    let b: f64 = 63.5;

    println!("add({}, {}) = {}", a, b, simple_math::add(a, b));
    println!("sub({}, {}) = {}", a, b, simple_math::sub(a, b));
    println!("mul({}, {}) = {}", a, b, simple_math::mul(a, b));
    println!("div({}, {}) = {}", a, b, simple_math::div(a, b));
    println!("rem({}, {}) = {}", a, b, simple_math::rem(a, b));
}
