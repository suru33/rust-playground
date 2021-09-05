mod rectangle;
mod square;
mod triangle;

pub struct Triangle {
    pub side1: f32,
    pub side2: f32,
    pub side3: f32,
}

pub struct Square {
    pub side: f32,
}

pub struct Rectangle {
    pub length: f32,
    pub width: f32,
}

pub trait Summary {
    fn name(&self) -> String;
}
