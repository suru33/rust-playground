use std::env;

use arg_reader;

fn main() {
    let mut args = env::args();

    args.next();

    let a: f64 = arg_reader::read_arg::<f64>(args.next(), 1);
    let b: f64 = arg_reader::read_arg::<f64>(args.next(), 2);

    println!("{}", a + b);
}
