pub fn square(num: i32) -> i32 {
    num * num
}

pub fn cube(num: i32) -> i32 {
    num * num * num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square() {
        assert_eq!(square(10), 100);
        assert_eq!(square(-5), 25);
    }

    #[test]
    fn test_cube() {
        assert_eq!(cube(10), 1000);
        assert_eq!(cube(-5), -125);
    }
}