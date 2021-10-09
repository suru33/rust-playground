## Documentation

[Original Doc](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html)

### Doc-tests

To run tests from documentation you need to have `lib.rs`

```
   Doc-tests crates_io_test

running 5 tests
test src/simple_math.rs - simple_math::mul (line 50) ... ok
test src/simple_math.rs - simple_math::div (line 69) ... ok
test src/simple_math.rs - simple_math::add (line 12) ... ok
test src/simple_math.rs - simple_math::rem (line 88) ... ok
test src/simple_math.rs - simple_math::sub (line 31) ... ok
```

### Doc-template

```rust
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
```

To generate and open documentation

```shell
cargo doc --open
```

Doc for simple_math module:
![img-1](/docs/img-1.png)