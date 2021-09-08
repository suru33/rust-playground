pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// Tests from DOC
/// Function divides two numbers.
///
/// # Examples
///
/// ```rust
/// use ex_19_more_tests::arithmetic;
///
/// let result = arithmetic::div(10, 2);
/// assert_eq!(result, 5_f64);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// use ex_19_more_tests::arithmetic;
///
/// arithmetic::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> f64 {
    if b == 0 {
        panic!("Divide by 0 error");
    }
    a as f64 / b as f64
}