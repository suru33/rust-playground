use std::fmt;
use std::fmt::Formatter;

use crate::geometry::Mensuration;
use crate::shapes::{Square, Summary};

impl fmt::Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Square<side: {}>", self.side)
    }
}

impl Mensuration for Square {
    fn validate(&self) {
        if self.side <= 0.0 {
            panic!("Not a valid Square: {}", self);
        }
    }

    fn area(&self) -> f32 {
        self.side * self.side
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.side
    }
}

impl Summary for Square {
    fn name(&self) -> String {
        String::from("Square")
    }
}