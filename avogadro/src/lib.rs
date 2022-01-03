//! # AVR-Avogadro
//!
//! Just another avr simulator library built in Rust. Simulator GUI is
//! written in C++ using Qt5 framework.
//!
//! See `doc/README.md` for more technical details.
#[macro_use]
extern crate log;
/// # Core
/// Main functions and components of the simulator.
pub mod core;
/// # FFI
/// Functions exposed to the C API, including a JNI interface for android
pub mod ffi;
