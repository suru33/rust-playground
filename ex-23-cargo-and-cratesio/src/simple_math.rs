//! Arithmetic functions
//!
//! This is a sample module with basic arithmetic functions

/// Adds 2 f64 numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: f64
///
/// # Examples
///
/// ```
/// use crates_io_test::simple_math;
/// assert_eq!(simple_math::add(45 as f64, 10 as f64), 55 as f64);
/// ```
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts 2 f64 numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: f64
///
/// # Examples
///
/// ```
/// use crates_io_test::simple_math;
/// assert_eq!(simple_math::sub(45 as f64, 10 as f64), 35 as f64);
/// ```
pub fn sub(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies 2 f64 numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: f64
///
/// # Examples
///
/// ```
/// use crates_io_test::simple_math;
/// assert_eq!(simple_math::mul(4 as f64, 10 as f64), 40 as f64);
/// ```
pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides 2 f64 numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: f64
///
/// # Examples
///
/// ```
/// use crates_io_test::simple_math;
/// assert_eq!(simple_math::div(45 as f64, 10 as f64), 4.5);
/// ```
pub fn div(a: f64, b: f64) -> f64 {
    a / b
}

/// Get reminder for 2 f64 numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: f64
///
/// # Examples
///
/// ```
/// use crates_io_test::simple_math;
/// assert_eq!(simple_math::rem(45 as f64, 10 as f64), 5 as f64);
/// ```
pub fn rem(a: f64, b: f64) -> f64 {
    a % b
}
