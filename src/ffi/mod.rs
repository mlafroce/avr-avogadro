#[cfg(target_os = "android")]
/// C API for JNI
pub mod android;
/// C API to interact with the MCU
pub mod mcu_wrapper;
