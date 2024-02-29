# Basic Grep CLI in Rust

This is a basic Grep CLI implementation in Rust. It matches the main Regex expressions. This is a small research project.

## How to run

1. Ensure you have `cargo (1.76)` installed locally
2. In the terminal run `echo "sentence" | ./your_grep.sh -E "regex"` where `regex` is the pattern to match

### Example

Run `echo "grep 101" | ./your_grep.sh -E "(\w+ \d\d\d)"`. The exit code will be the result.

## How to test

Run `cargo test` to run the tests
