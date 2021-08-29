// the name `commons` is from `Cargo.toml -> [package].name`
use commons::hello;
use commons::utils::numbers;
use commons::utils::text;

fn main() {
    println!("Hello, world!");
    hello();
    let x = 100;
    println!("is x = {} odd ? {}", x, numbers::is_odd(x));

    let s = String::from("some text");
    println!("len({}) = {}", s, text::str_len(&s));
}
