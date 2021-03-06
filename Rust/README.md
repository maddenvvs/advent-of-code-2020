# 🎄⭐️ [Advent of Code 2020](https://adventofcode.com/2020)
## Solutions in 🦀 [Rust](https://www.rust-lang.org/)

This project builds with [Cargo](https://docs.rust-lang.org/cargo).


## Useful commands

Run the test suite:

```sh
$ cargo test
```

Run the test suite (`release` profile):

```sh
$ cargo test --release
```

Run solution for a day 1:
```sh
$ cargo run -- 1 ../input/day-01.input
```

Run solution for a day 17 (`release` profile):
```sh
$ cargo run --release -- 17 ../input/day-17.input
```

Run benchmark tests:
```sh
$ cargo bench
```

## TODO
- [x] Add benchmarks
    Use [Criterion](https://github.com/bheisler/criterion.rs) to measure performance.
- [x] Try `RUSTFLAGS='-C target-cpu=native' cargo run --release` to test performance difference
    Didn't find any significant difference in performance.