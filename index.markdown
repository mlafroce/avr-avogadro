---
layout: page
title: Home
---

*AVR-Avogadro* is an open source AVR simulator made in **Rust** and **C++**

Core library is written in Rust and exposes an API used by a GUI written in C++ (**QT5**).

This simulator aims to be suited for performance analysis.


# Building from sources

Currently only linux target is supported.  To build from sources you need

* Rust toolchain (includes *rustc* and *cargo*)

* GCC toolchain  (*g++* needed for GUI)

* CMake (>= 3.9) (Cargo build instructions use cmake to build GUI)

Download Rust compiler following instructions on their [site](https://www.rust-lang.org/tools/install).

Install *GCC* and *Qt5* with the following commands

~~~
sudo apt install build-essential cmake qt5-default
~~~

Compile with `cargo build` Rust's package manager will download and build dependencies.

Run `cargo run` to run the application.

## Tests and linter

Run `cargo test` to build and run unit tests.

*Clippy* is used as Rust linter. Install and run with

~~~
rustup component add clippy
cargo clippy
~~~
