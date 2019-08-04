use crate::core::mcu::Mcu;
use std::ptr;

#[no_mangle]
pub extern fn mcu_create(p_mcu: *mut *mut Mcu) {
    unsafe {
        let mcu = Box::new(Mcu::new());
        *p_mcu = Box::into_raw(mcu);
    }
}

#[no_mangle]
pub extern fn mcu_destroy(p_mcu: *mut *mut Mcu) {
    unsafe {
        Box::from_raw(*p_mcu);
    }
}

#[no_mangle]
pub extern "C" fn mcu_step(p_mcu: *mut Mcu) {
    unsafe {
        (*p_mcu).step();
    }
}

#[no_mangle]
pub fn mcu_load_memory(p_mcu: *mut Mcu, p_memory: *const u8, memory_size: usize) {
    unsafe {
        let mut rust_mem = Vec::with_capacity(memory_size);
        rust_mem.set_len(memory_size);
        ptr::copy(p_memory, rust_mem.as_mut_ptr(), memory_size);
        (*p_mcu).load_memory(&rust_mem);
    }
}

#[no_mangle]
pub fn mcu_get_program_counter(p_mcu: *mut Mcu) -> u16 {
    unsafe {
        (*p_mcu).get_program_counter()
    }
}

#[no_mangle]
pub fn mcu_get_register(p_mcu: *mut Mcu, reg_num: u8) -> u8 {
    unsafe {
        (*p_mcu).get_register(reg_num)
    }
}

#[no_mangle]
pub fn mcu_set_register(p_mcu: *mut Mcu, reg_num: u8, value: u8) {
    unsafe {
        (*p_mcu).set_register(reg_num, value);
    }
}
/*
pub fn get_flags(p_mcu: *mut Mcu) {
    unsafe {
        (*p_mcu).get_flags();
    }
}

pub fn set_flags(p_mcu: *mut Mcu) {
    unsafe {
        (*p_mcu).get_flags();
    }
}
*/