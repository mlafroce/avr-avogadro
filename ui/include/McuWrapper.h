#ifndef MCU_WRAPPER_H
#define MCU_WRAPPER_H
#include <cstddef>

class McuWrapper {
public:
    explicit McuWrapper(void* mcu);
    void step();
    void getRegisterArray(char* buffer);
    void setRegisterArray(char* buffer);
    void setRegister(char registerId, char value);
    short getProgramCounter();
    void setProgramCounter(short value);
    short getCurrentInstruction();
    void displayCurrentInstruction(char* buffer, std::size_t size);
    void loadFile(const char* filename);
private:
    void* mcu;
};

#endif // MCU_WRAPPER_H
