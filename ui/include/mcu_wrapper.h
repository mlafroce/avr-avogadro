#ifndef MCU_WRAPPER_H
#define MCU_WRAPPER_H

typedef void Mcu;

extern "C" {
	void mcu_step(Mcu* mcu);
	void mcu_get_register_array(Mcu* mcu, char* buffer);
	void mcu_set_register_array(Mcu* mcu, char* buffer);
}

#endif // MCU_WRAPPER_H