mod core;
mod ffi;

use crate::core::mcu::Mcu;

use libc::{c_char, c_void};
use std::env;
use std::ffi::CString;

extern "C" { 
    // this is rustified prototype of the function from our C library
    fn run_avogadro_gui(argc: usize, argv: *const *const c_char, mcu: *const c_void); 
}

/// Executes ADD r1, r2
/// 2 + 3 = 5
fn main() {
    let mut mcu = Mcu::new();
    mcu.set_register(1, 2);
    mcu.set_register(2, 3);
    let memory_data = vec![0x12, 0x0C];
    mcu.load_memory(&memory_data);
    mcu.step();
    println!("2 + 3 = {}", mcu.get_register(1));
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    unsafe {
    	run_avogadro_gui(c_args.len(), c_args.as_ptr(), &mcu as *const Mcu as *const c_void);
    }
}
