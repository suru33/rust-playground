use std::fmt;
use std::fmt::Formatter;

use crate::geometry::Mensuration;
use crate::shapes::{Summary, Triangle};

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Triangle<side1: {}, side2: {}, side3:{}>",
            self.side1, self.side2, self.side3
        )
    }
}

impl Mensuration for Triangle {
    fn validate(&self) {
        if self.side1 <= 0.0 || self.side2 <= 0.0 || self.side3 <= 0.0 {
            panic!("Not a valid triangle: {}", self)
        }
    }

    fn area(&self) -> f32 {
        self.validate();
        let p = (self.side1 + self.side2 + self.side3) / 2.0;
        (p * (p - self.side1) * (p - self.side2) * (p - self.side3)).sqrt()
    }

    fn perimeter(&self) -> f32 {
        self.validate();
        self.side1 + self.side2 + self.side3
    }
}

impl Summary for Triangle {
    fn name(&self) -> String {
        String::from("Triangle")
    }
}
