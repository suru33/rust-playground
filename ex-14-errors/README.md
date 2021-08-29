# Rust errors

```rust
// simple instant crash
// exit code 101
panic!("Crash......"); 
```

Backtrace the panic

```rust
fn main() {
    println!("Hello, world!");

    let v = vec![100, 12, 43, 13];
    v[10]; // index out of bounds
}
```

- `cargo run`
  ```shell
  λ: cargo run
  Compiling ex-14-errors v0.1.0 (/Users/suru/workspace/suru33/rust/rust-playground/ex-14-errors)
  Finished dev [unoptimized + debuginfo] target(s) in 0.41s
  Running `target/debug/ex-14-errors`
  Hello, world!
  thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 10', src/main.rs:9:5
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```
- `RUST_BACKTRACE=1 cargo run`
  ```shell
  λ: RUST_BACKTRACE=1 cargo run
  Finished dev [unoptimized + debuginfo] target(s) in 0.00s
  Running `target/debug/ex-14-errors`
  Hello, world!
  thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 10', src/main.rs:9:5
  stack backtrace:
  0: rust_begin_unwind
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
  1: core::panicking::panic_fmt
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
  2: core::panicking::panic_bounds_check
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
  3: <usize as core::slice::index::SliceIndex<[T]>>::index
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/slice/index.rs:184:10
  4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/slice/index.rs:15:9
  5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/alloc/src/vec/mod.rs:2428:9
  6: ex_14_errors::main
  at ./src/main.rs:9:5
  7: core::ops::function::FnOnce::call_once
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/ops/function.rs:227:5
  note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
  ```
- `RUST_BACKTRACE=full cargo run`
  ```shell
  λ: RUST_BACKTRACE=full cargo run
  Finished dev [unoptimized + debuginfo] target(s) in 0.00s
  Running `target/debug/ex-14-errors`
  Hello, world!
  thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 10', src/main.rs:9:5
  stack backtrace:
  0:        0x104f2f624 - std::backtrace_rs::backtrace::libunwind::trace::h03e89bdd1996f493
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
  1:        0x104f2f624 - std::backtrace_rs::backtrace::trace_unsynchronized::hb5af08e4de8ca785
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
  2:        0x104f2f624 - std::sys_common::backtrace::_print_fmt::h4eafc628aec41041
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/sys_common/backtrace.rs:67:5
  3:        0x104f2f624 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6a8908fa3ed6f9e8
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/sys_common/backtrace.rs:46:22
  4:        0x104f45b9c - core::fmt::write::h4be00f71c5582919
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/fmt/mod.rs:1110:17
  5:        0x104f2d61a - std::io::Write::write_fmt::h49e76926070788f1
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/io/mod.rs:1588:15
  6:        0x104f3118f - std::sys_common::backtrace::_print::h2f7b8ad2bea4f859
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/sys_common/backtrace.rs:49:5
  7:        0x104f3118f - std::sys_common::backtrace::print::h97402070cfceb1a3
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/sys_common/backtrace.rs:36:9
  8:        0x104f3118f - std::panicking::default_hook::{{closure}}::h1577f0656e419c0e
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:208:50
  9:        0x104f30c8d - std::panicking::default_hook::h1aef594179c4fd25
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:225:9
  10:        0x104f31890 - std::panicking::rust_panic_with_hook::h10bc487d002f6c42
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:622:17
  11:        0x104f31335 - std::panicking::begin_panic_handler::{{closure}}::hf4cfa78c105ce648
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:519:13
  12:        0x104f2fa98 - std::sys_common::backtrace::__rust_end_short_backtrace::h1df96a166e4351c4
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/sys_common/backtrace.rs:141:18
  13:        0x104f3129a - rust_begin_unwind
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:515:5
  14:        0x104f4c72f - core::panicking::panic_fmt::hea8fe6c9e0720810
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:92:14
  15:        0x104f4c6f6 - core::panicking::panic_bounds_check::h2a74ead3b5b4ded0
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/panicking.rs:69:5
  16:        0x104f18e16 - <usize as core::slice::index::SliceIndex<[T]>>::index::h2ddf8fd381ec8b26
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/slice/index.rs:184:10
  17:        0x104f18da7 - core::slice::index::<impl core::ops::index::Index<I> for [T]>::index::hfa89e6582991859f
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/slice/index.rs:15:9
  18:        0x104f18f7e - <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index::h211843d3697774cf
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/alloc/src/vec/mod.rs:2428:9
  19:        0x104f18422 - ex_14_errors::main::h4e811edce5efc00a
  at /Users/suru/workspace/suru33/rust/rust-playground/ex-14-errors/src/main.rs:9:5
  20:        0x104f184de - core::ops::function::FnOnce::call_once::h2c55ecc92bdf6beb
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/ops/function.rs:227:5
  21:        0x104f187b1 - std::sys_common::backtrace::__rust_begin_short_backtrace::h3df281857c8ba7cc
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/sys_common/backtrace.rs:125:18
  22:        0x104f18784 - std::rt::lang_start::{{closure}}::hf8b10ae57bac8838
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/rt.rs:49:18
  23:        0x104f31cd1 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h7adc4e20a318e389
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/core/src/ops/function.rs:259:13
  24:        0x104f31cd1 - std::panicking::try::do_call::h687af85ae86a6e14
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:401:40
  25:        0x104f31cd1 - std::panicking::try::hf74f2790da0186f3
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panicking.rs:365:19
  26:        0x104f31cd1 - std::panic::catch_unwind::hd14a420ea313edb8
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/panic.rs:434:14
  27:        0x104f31cd1 - std::rt::lang_start_internal::h4428f22d05a79c62
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/rt.rs:34:21
  28:        0x104f1875e - std::rt::lang_start::h9efe079657c9308a
  at /rustc/a178d0322ce20e33eac124758e837cbd80a6f633/library/std/src/rt.rs:48:5
  29:        0x104f18476 - _main
  ```