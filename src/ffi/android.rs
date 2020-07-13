use crate::core::mcu::Mcu;
use crate::core::mcu_factory::McuFactory;
use crate::ffi::mcu_wrapper;

use std::convert::TryInto;

#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jlong, jchar, jbyteArray};

    #[no_mangle]
    pub unsafe extern fn Java_com_mlafroce_avogadro_wrapper_AvrAvogadroWrapper_createMcu(
            env: JNIEnv, _: JClass, java_pattern: JString) -> jlong {
        let mcu_box = Box::new(McuFactory::create("attiny85"));
        Box::into_raw(mcu_box) as i64
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mlafroce_avogadro_wrapper_AvrAvogadroWrapper_freeMcu(
            env: JNIEnv, _: JClass, ptr: jlong) {
        let drop = unsafe { Box::from_raw(ptr as *mut Mcu) };
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mlafroce_avogadro_wrapper_AvrAvogadroWrapper_mcuLoadProgramMemory(
            env: JNIEnv, _: JClass, ptr: jlong, program_memory: jbyteArray) {
        let rust_ptr = ptr as *mut Mcu;
        let rust_memory = env.convert_byte_array(program_memory).unwrap();
        mcu_wrapper::mcu_load_program_memory(&mut *rust_ptr, rust_memory.as_ptr(), rust_memory.len());
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mlafroce_avogadro_wrapper_AvrAvogadroWrapper_mcuStep(
            env: JNIEnv, _: JClass, ptr: jlong) {
        let rust_ptr = ptr as *mut Mcu;
        mcu_wrapper::mcu_step(&mut *rust_ptr);
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mlafroce_avogadro_wrapper_AvrAvogadroWrapper_mcuGetRegister(
            env: JNIEnv, _: JClass, ptr: jlong, reg_num: jchar) -> jchar {
        let rust_ptr = ptr as *mut Mcu;
        mcu_wrapper::mcu_get_register(&mut *rust_ptr, reg_num.try_into().unwrap()).into()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mlafroce_avogadro_wrapper_AvrAvogadroWrapper_mcuSetRegister(
            env: JNIEnv, _: JClass, ptr: jlong, reg_num: jchar, value: jchar) {
        let rust_ptr = ptr as *mut Mcu;
        mcu_wrapper::mcu_set_register(&mut *rust_ptr, reg_num.try_into().unwrap(), value.try_into().unwrap());
    }
}
