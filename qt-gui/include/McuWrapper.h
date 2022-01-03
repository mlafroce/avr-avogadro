#ifndef MCU_WRAPPER_H
#define MCU_WRAPPER_H
#include <cstddef>
#include <vector>

class McuWrapper {
public:
    explicit McuWrapper(void* mcu);
    void step() const;
    void getRegisterArray(const unsigned char* buffer) const;
    void setRegisterArray(const unsigned char* buffer) const;
    void setRegister(char registerId, char value) const;
    short getProgramCounter() const;
    void setProgramCounter(short value) const;
    short getStackPointer() const;
    short getCurrentInstruction() const;
    void displayCurrentInstruction(const char* buffer, std::size_t size) const;
    void loadBinFile(const char* filename, bool isProgram) const;
    void loadIhexFile(const char* filename) const;
    void getDataMemory(std::vector<char>& buffer) const;
    void getProgramMemory(std::vector<char>& buffer) const;
    unsigned char getDataByte(short int);
    unsigned char getFlags() const;
private:
    void* mcu;
};

#endif // MCU_WRAPPER_H
