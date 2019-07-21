mod core;

use crate::core::mcu::Mcu;

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
}
