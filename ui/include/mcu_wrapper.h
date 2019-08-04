#ifndef MCU_WRAPPER_H
#define MCU_WRAPPER_H

typedef void Mcu;

extern "C" {
	void mcu_step(Mcu* mcu);
}

#endif // MCU_WRAPPER_H