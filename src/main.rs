#[macro_use]
extern crate log;
extern crate env_logger;

mod core;
mod ffi;

use crate::core::mcu::Mcu;

use libc::{c_char, c_void};
use std::ffi::CString;

#[link(name = "avogadrogui")]
#[link(name = "Qt5Widgets")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Core")]
#[link(name = "stdc++")]
extern "C" { 
    fn run_avogadro_gui(argc: usize, argv: *const *const c_char, mcu: *const c_void); 
}

fn main() {
    let mcu = Mcu::new();
    // create a vector of zero terminated strings
    let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
    // convert the strings to raw pointers
    let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
    // Init logger
    env_logger::init();
    unsafe {
    	run_avogadro_gui(c_args.len(), c_args.as_ptr(), &mcu as *const Mcu as *const c_void);
    }
}
