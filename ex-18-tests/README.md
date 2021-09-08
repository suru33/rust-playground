# Rust unit testing

### Sample test case

```rust
fn x() -> String {
    String::from("Hello World")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic] // function should panic
    #[ignore] // this test is ignored with `cargo test`
    fn test_x() {
        assert_eq!(x(), String::from("Hello World"));
    }
}
```

```shell
# To run tests
caro test

# To run only ignored tests
cargo test -- --ignored
```