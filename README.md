# AVR-Avogadro

AVR simulator made in Rust and C++

Core library is in Rust and will expose an API for GUI in C++ (QT).

This simulator aims to be suited for performance analysis.

## Requeriments

* Rust compiler

* Gcc

* CMake (>= 3.9)

## Instructions

### Ubuntu

Download Rust compiler following instructions on their site.

Install gcc and Qt with the following commands

~~~
apt install build-essential cmake qt5-default
~~~

Just run `cargo build` and rust's package manager will download and build dependencies.

Run `cargo run` to run the application.

## Linter

Clippy is used as Rust linter.

~~~
rustup component add clippy
cargo clippy
~~~