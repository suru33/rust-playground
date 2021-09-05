use std::fmt::Display;

use geometry::Mensuration;
use shapes::Rectangle;
use shapes::Square;
use shapes::Triangle;

use crate::shapes::Summary;

mod geometry;
mod shapes;

fn main() {
    let t1 = Triangle {
        side1: 15.0,
        side2: 14.0,
        side3: 10.0,
    };

    println!("t1: {}", t1);
    println!("t1 area: {}", t1.area());
    println!("t1 perimeter: {}", t1.perimeter());

    let s1 = Square { side: 19.73 };

    println!("s1: {}", s1);
    println!("s1 area: {}", s1.area());
    println!("s1 perimeter: {}", s1.perimeter());

    let r1 = Rectangle {
        length: 8.01,
        width: 43.0,
    };

    info(&r1);
}

fn info(shape: &(impl Mensuration + Summary + Display)) {
    println!("Shape: {}", shape.name());
    println!("{}", shape);
    println!("Area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}

fn info2<T: Mensuration + Summary + Display>(shape: &T) {
    println!("Shape: {}", shape.name());
    println!("{}", shape);
    println!("Area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}

fn info3<T>(shape: &T) -> ()
where
    T: Mensuration + Summary + Display,
{
    println!("Shape: {}", shape.name());
    println!("{}", shape);
    println!("Area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}
