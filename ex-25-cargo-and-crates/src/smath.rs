/// Add two numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: <T as Add<T>>::Output
///
/// # Examples
///
/// ```
///use ex_25_cargo_and_crates::smath::add;
/// assert_eq!(add(5 as usize, 25 as usize), 30 as usize);
/// assert_eq!(add(5, 25), 30);
/// assert_eq!(add(1.0, 5.0), 6.0);
/// ```
pub fn add<T>(a: T, b: T) -> T::Output
    where T: std::ops::Add {
    a + b
}

/// Subtract two numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: <T as Sub<T>>::Output
///
/// # Examples
///
/// ```
/// use ex_25_cargo_and_crates::smath::sub;
/// assert_eq!(sub(1.0, 5.0), -4.0);
/// assert_eq!(sub(10, 5), 5);
/// assert_eq!(sub(10 as u8, 1 as u8), 9 as u8);
/// ```
pub fn sub<T>(a: T, b: T) -> T::Output
    where T: std::ops::Sub {
    a - b
}

/// Multiply two numbers
///
/// # Arguments
///
/// * `a`: number 1
/// * `b`: number 2
///
/// returns: <T as Mul<T>>::Output
///
/// # Examples
///
/// ```
/// use ex_25_cargo_and_crates::smath::mul;
/// assert_eq!(mul(1.0, 5.0), 5.0);
/// assert_eq!(mul(10, 5), 50);
/// assert_eq!(mul(10 as u8, 1 as u8), 10 as u8);
/// ```
pub fn mul<T>(a: T, b: T) -> T::Output
    where T: std::ops::Mul {
    a * b
}
