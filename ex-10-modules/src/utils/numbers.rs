use super::c;

pub fn is_even(i: u32) -> bool {
    if i % 2 == 0 {
        true
    } else {
        false
    }
}

pub fn is_odd(i: u32) -> bool {
    c::print_from_c();
    !is_even(i)
}
