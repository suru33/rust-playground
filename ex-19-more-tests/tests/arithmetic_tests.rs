use ex_19_more_tests::arithmetic;

#[test]
fn test_add() {
    assert_eq!(arithmetic::add(3, 2), 5);
}

#[test]
fn test_sub() {
    assert_eq!(arithmetic::sub(3, 2), 1);
}

#[test]
fn test_mul() {
    assert_eq!(arithmetic::mul(3, 2), 6);
}

#[test]
fn test_div() {
    assert_eq!(arithmetic::div(3, 2), 1.5_f64);
}

#[test]
#[should_panic]
fn test_div_by_zero() {
    arithmetic::div(12, 0);
}