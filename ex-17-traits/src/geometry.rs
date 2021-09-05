pub trait Mensuration {
    fn validate(&self);
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

