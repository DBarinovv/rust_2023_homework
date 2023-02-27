# Solving and submitting problems

This document describes how to work with course problems.

## Rules

- **Do not cheat**. At the end of the course, problems will be checked for plagiarism.

## Solving workload

- Read the problem statement.
- Read all tests located in `tests/tests.rs` to understand what you need to code.
- Code a working solution.
- Run `cargo test` and make sure it's working.
- Run `cargo clippy` to lint the code.
- Run `cargo fmt` to format the code.
- Answer bonus questions and complete bonus levels if there are any, and you know how to do them.
- *There will be CI step soon.*

## Cheatsheet of useful commands

- `cargo build` - build the project.
- `cargo test` - (re)build and run all tests. You must be in the problem's directory.
- `cargo test -j 1` - Run tests sequentially. By default, cargo runs tests in parallel, up to your number of CPUs.
- `cargo test -- --nocapture` - this will show you the runtime output of the tests. Useful when debugging.
- `cargo fmt` - format the code according to the Rust code style.
- `cargo clippy` - lint the code; it will show you some cool ways to improve your code find typos and even bugs.
- `cargo fix` - automatically fix warnings of the compiler.
- `cargo run` - run the binary if it's the binary project.
- `cargo run -- *some_arguments*` - run the binary if it's the binary project with specified arguments.
- `cargo check` - check the code for errors; the same as build, but faster because it skips the last code generation step.
- `cargo expand` - expand the code; useful when debugging macros.
- `cargo criterion` - execute benchmarks. You'll need to `cargo install cargo-criterion`
