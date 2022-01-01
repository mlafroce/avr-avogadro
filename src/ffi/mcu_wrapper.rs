//! This module contains the C API to interact with the MCU
use crate::core::mcu::Mcu;
use crate::core::register_bank::Flags;

use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr;

/// Calls `Mcu::step()`, executing one clock cycle of the mcu
#[no_mangle]
pub extern "C" fn mcu_step(p_mcu: &mut Mcu) {
    p_mcu.step();
}

/// Calls `Mcu::load_memory_from_file(filename)`
/// Returns 0 if memory was loaded correctly
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
/// `p_filename` must be a valid C string
#[no_mangle]
pub unsafe fn mcu_load_file(p_mcu: &mut Mcu, p_filename: *const c_char, is_program: bool) -> u8 {
    let filename = CStr::from_ptr(p_filename).to_str();
    if p_mcu.load_from_file(filename.unwrap(), is_program).is_ok() {
        0
    } else {
        warn!("Error");
        1
    }
}

/// Creates a Rust vector with size `memory_size` and contents of `p_memory`
/// and calls `Mcu::load_data_memory`
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
/// `p_memory` should be a char array with  size equals or larger than memory_size
#[allow(clippy::unsound_collection_transmute)]
#[no_mangle]
pub unsafe fn mcu_load_data_memory(p_mcu: &mut Mcu, p_memory: *const u8, memory_size: usize) {
    let mut buffer: Vec<MaybeUninit<u8>> = Vec::with_capacity(memory_size);
    buffer.set_len(memory_size);
    ptr::copy_nonoverlapping(p_memory, buffer.as_mut_ptr() as *mut u8, memory_size);
    let init_buf = std::mem::transmute::<_,Vec<u8>>(buffer);
    p_mcu.load_data_memory(&init_buf);
}

/// Creates a Rust vector with size `memory_size` and contents of `p_memory`
/// and calls `Mcu::load_program_memory`
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
/// `p_memory` should be a char array with  size equals or larger than memory_size
#[allow(clippy::unsound_collection_transmute)]
#[no_mangle]
pub unsafe fn mcu_load_program_memory(p_mcu: &mut Mcu, p_memory: *const u8, memory_size: usize) {
    let mut buffer: Vec<MaybeUninit<u8>> = Vec::with_capacity(memory_size);
    buffer.set_len(memory_size);
    ptr::copy_nonoverlapping(p_memory, buffer.as_mut_ptr() as *mut u8, memory_size);
    let init_buf = std::mem::transmute::<_,Vec<u8>>(buffer);
    p_mcu.load_program_memory(&init_buf);
}

/// Gets data stored in a single register
#[no_mangle]
pub fn mcu_get_register(p_mcu: &Mcu, reg_num: u8) -> u8 {
    p_mcu.get_register(reg_num)
}

/// Sets data into register `regnum`
#[no_mangle]
pub fn mcu_set_register(p_mcu: &mut Mcu, reg_num: u8, value: u8) {
    p_mcu.set_register(reg_num, value);
}

/// Puts registers data into a buffer
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
/// `buffer` must be a char array with at least 32 bytes
///
#[no_mangle]
pub unsafe fn mcu_get_register_array(p_mcu: &Mcu, buffer: *mut u8) {
    let registers = p_mcu.get_register_array();
    ptr::copy_nonoverlapping(registers.as_ptr(), buffer, 32);
}

#[no_mangle]
pub fn mcu_set_register_array(p_mcu: &mut Mcu, reg_array: [u8; 32]) {
    p_mcu.set_register_array(reg_array);
}

#[no_mangle]
pub fn mcu_get_program_counter(p_mcu: &Mcu) -> u16 {
    p_mcu.get_program_counter()
}

#[no_mangle]
pub fn mcu_get_stack_pointer(p_mcu: &Mcu) -> u16 {
    p_mcu.get_stack_pointer()
}

#[no_mangle]
pub fn mcu_set_program_counter(p_mcu: &mut Mcu, value: u16) {
    p_mcu.set_program_counter(value);
}

#[no_mangle]
pub fn mcu_get_current_instruction(p_mcu: &Mcu) -> u16 {
    p_mcu.get_current_instruction()
}

#[no_mangle]
pub fn mcu_get_data_byte(p_mcu: &Mcu, address: u16) -> u8 {
    p_mcu.get_data_byte(address)
}

/// Puts registers data into a buffer
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
/// `buffer` must be a char array with `buf_size` size
///
#[no_mangle]
pub unsafe fn mcu_display_current_instruction(p_mcu: &Mcu, c_buffer: *mut u8, buf_size: usize) {
    let mut string_buf = String::new();
    p_mcu.display_current_instruction(&mut string_buf);
    let bytes_to_copy = std::cmp::min(buf_size - 1, string_buf.len());
    ptr::copy_nonoverlapping(string_buf.as_ptr(), c_buffer, bytes_to_copy);
    *(c_buffer.add(bytes_to_copy)) = 0;
}

/// Gets memory bank size
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
///
#[no_mangle]
pub unsafe fn mcu_get_data_size(p_mcu: &Mcu) -> usize {
    p_mcu.get_data_size()
}

/// Gets memory bank contents
/// # Safety
///
/// `p_mcu` must be a pointer to a valid Mcu
/// `buffer` must be a char array with `buf_size` size
///
#[no_mangle]
pub unsafe fn mcu_get_data_memory(p_mcu: &Mcu, c_buffer: *mut u8, buf_size: usize) {
    p_mcu.get_data_memory(c_buffer, buf_size);
}

#[no_mangle]
pub fn mcu_get_flags(p_mcu: &Mcu) -> u8 {
    p_mcu.get_flags().into()
}

#[no_mangle]
pub fn mcu_set_flags(p_mcu: &mut Mcu, flags: Flags) {
    p_mcu.set_flags(flags);
}
