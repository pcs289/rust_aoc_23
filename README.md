# 🦀🎄 Rust - Advent Of Code 2023

This repository contains my particular implementation of (hopefully all) daily challenges from [Advent of Code 2023](https://adventofcode.com). This is done as a learning exercise to dive deeper into [Rust programming language](https://rust-lang.org/learn).

## Code Structure

The repository is organized as a Rust workspace with multiple Rust crates named `day_X` (a number between `1` and `25`) each representing a day of December in the advent calendar 🎄🦀.

Each daily challenge contains:
- Cargo File: `day_X/Cargo.toml`
- Input File: `day_X/input.txt`
- Library File: `day_X/src/lib.rs`
- Binary File: `day_X/src/main.rs`

## Run `day_X` binary
```shell
cargo run --bin day_1
```

## Run `day_X` tests with printed output
```shell
cargo test --lib day_1 -- --nocapture
```

## Run all tests
```shell
cargo test --lib
```
