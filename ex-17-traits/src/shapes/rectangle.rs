use std::fmt;
use std::fmt::Formatter;

use crate::geometry::Mensuration;
use crate::shapes::{Rectangle, Summary};

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Rectangle<length: {}, width: {}>",
            self.length, self.width
        )
    }
}

impl Mensuration for Rectangle {
    fn validate(&self) {
        if self.length <= 0.0 || self.width <= 0.0 {
            panic!("Noe a valid Rectangle: {}", self);
        }
    }

    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.length)
    }
}

impl Summary for Rectangle {
    fn name(&self) -> String {
        String::from("Rectangle")
    }
}
