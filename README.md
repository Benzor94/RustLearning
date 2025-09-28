# RustLearning

Repository for random small Rust learning projects, including stuff in the Rust book. Probably nothing interesting to anyone besides myself.

# Cargo cheatsheet

## Initializing projects

* Initializing project: `cargo new <project_name>`, defaults to binary
* Initializing binary project: `cargo new --bin <project_name>`, same as above
* Initializing library project: `cargo new --lib <project_name>`
* Initializing project into existing directory: `cargo init [--bin/--lib]`

## Compiling and running

* Build project: `cargo build`
* Run project: `cargo run`
  - With release profile: `cargo run --release`
  - With quiet mode: `cargo run -q`
* Check for compile-time errors: `cargo check`
(to be expanded)
