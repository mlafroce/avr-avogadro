# AVR-Avogadro

[![Build Status](https://travis-ci.org/mlafroce/avr-avogadro.svg?branch=development)](https://travis-ci.org/mlafroce/avr-avogadro)
[![codecov](https://codecov.io/gh/mlafroce/avr-avogadro/branch/development/graph/badge.svg)](https://codecov.io/gh/mlafroce/avr-avogadro)

AVR simulator made in Rust and C++

Core library is in Rust and will expose an API for GUI in C++ (QT).

This simulator aims to be suited for performance analysis.

# Building from sources

## Dependencies

Currently only linux target is supported.  To build from sources you need

* Rust toolchain (includes *rustc* and *cargo*)

* GCC toolchain  (*g++* needed for GUI)

* CMake (>= 3.9) (Cargo build instructions use *cmake* to build GUI)

From a terminal, you can clone the repo with 

~~~
git clone --recurse-submodules https://github.com/mlafroce/avr-avogadro.git
~~~

This will download [QHexEdit2](https://github.com/Simsys/qhexedit2/) as a dependency.

If you choose "Download as a zip", copy QHexEdit2 sources into `ui/3rd-party/qhexedit2`

To get their last version, download Rust compiler following instructions on their [site](https://www.rust-lang.org/tools/install).

Install *GCC* and *Qt5* with the following commands

~~~
sudo apt install build-essential cmake qt5-default
~~~

## Build

Compile with `cargo build` Rust's package manager will download and build dependencies.

Run `cargo run` to run the application.

## Tests and linter

Run `cargo test` to build and run *unit tests*.

*Clippy* is used as Rust linter. Install and run with

~~~
rustup component add clippy
cargo clippy
~~~

## Examples

Small avr examples are in `examples` folder.

You can compile them running `make` or manually using `avr-gcc`.

Install with

~~~
sudo apt install gcc-avr
~~~

# License

This project is licensed under the MIT License, see the [LICENSE.md](https://github.com/mlafroce/avr-avogadro/blob/development/LICENSE.md) in the repository for details

# Acknowledgements

Thanks to [Luciano Rabassa](https://www.facebook.com/profile.php?id=100015059700810) for his contributions as a Beta tester.

Thanks to @Simsys for his awesome HexEditor Widget!
