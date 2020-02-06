#include "McuWrapper.h"

extern "C" {
    void mcu_step(void* mcu);
    void mcu_get_register_array(void* mcu, unsigned char* buffer);
    void mcu_set_register_array(void* mcu, unsigned char* buffer);
    void mcu_set_register(void* mcu, char registerId, char value);
    short mcu_get_program_counter(void* mcu);
    void mcu_set_program_counter(void* mcu, short value);
    short mcu_get_stack_pointer(void* mcu);
    short mcu_get_current_instruction(void* mcu);
    void mcu_display_current_instruction(void* mcu, char* uffer, size_t size);
    void mcu_load_file(void* mcu, const char* filename);
}

McuWrapper::McuWrapper(void* mcu) : mcu(mcu) {}

void McuWrapper::step() {
    mcu_step(this->mcu);
}

void McuWrapper::getRegisterArray(unsigned char* buffer) {
    mcu_get_register_array(this->mcu, buffer);
}

void McuWrapper::setRegisterArray(unsigned char* buffer) {
    mcu_set_register_array(this->mcu, buffer);
}

void McuWrapper::setRegister(char registerId, char value) {
    mcu_set_register(this->mcu, registerId, value);
}

short McuWrapper::getProgramCounter() {
    return mcu_get_program_counter(this->mcu);
}

void McuWrapper::setProgramCounter(short value) {
    mcu_set_program_counter(this->mcu, value);
}

short McuWrapper::getStackPointer() {
    return mcu_get_stack_pointer(this->mcu);
}

short McuWrapper::getCurrentInstruction() {
    return mcu_get_current_instruction(this->mcu);
}

void McuWrapper::displayCurrentInstruction(char* buffer, std::size_t size) {
    mcu_display_current_instruction(this->mcu, buffer, size);
}

void McuWrapper::loadFile(const char* filename) {
    mcu_load_file(this->mcu, filename);
}
