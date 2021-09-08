pub fn is_even(num: &i32) -> bool {
    num % 2 == 0
}

pub fn is_odd(num: &i32) -> bool {
    !is_even(num)
}

pub fn div(num1: i32, num2: i32) -> f64 {
    if num2 == 0 {
        panic!("num2 should not be zero")
    }
    num1 as f64 / num2 as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(&100), true);
        assert_eq!(is_even(&101), false);
    }

    #[test]
    fn test_is_odd() {
        assert_eq!(is_odd(&201), true);
        assert_eq!(is_odd(&100), false);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(10, 2), 5_f64);
    }

    #[test]
    #[should_panic]
    fn test_div_panic() {
        div(100, 0);
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn test_div_panic_duplicate() {
        div(100, 0);
    }
}