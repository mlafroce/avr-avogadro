use super::mcu::Mcu;

pub struct McuFactory;

impl McuFactory {
    pub fn create(mcu_name: &str) -> Mcu {
        match mcu_name {
            "attiny85" => Mcu::new(512, 8 * 1024),
            _ => unimplemented!(),
        }
    }
}
