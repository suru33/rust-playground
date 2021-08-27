#[derive(Debug)] // to print the struct to console
struct Rectangle {
    width: u32,
    height: u32,
}

// defining methods for Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn can_hold(&self, another_rectangle: &Rectangle) -> bool {
        if self.width > another_rectangle.width && self.height > another_rectangle.height {
            true
        } else {
            false
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 23,
        height: 8,
    };

    let r2 = Rectangle::from(10, 12);
    let r3 = Rectangle::from(10, 5);

    println!("r1: {:#?}", r1);
    println!("r1 area: {}", area(&r1));
    println!("r1 area: {}", r1.area()); // appropriate way
    println!("can r1 fits r2? {}", r1.can_hold(&r2));
    println!("can r1 fits r3? {}", r1.can_hold(&r3));
}

// one way
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
