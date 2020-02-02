#ifndef MAIN_WINDOW_H
#define MAIN_WINDOW_H

#include <QMainWindow>
#include "McuWrapper.h"

class MainWindow : public QMainWindow {
public:
    explicit MainWindow(QMainWindow *parent, void* mcu);
    virtual ~MainWindow();
private:
    void mcuStep();
    std::string getSelectedFilename();    
    void loadFile();
    void connectEvents();
    void updateMcuStatus();
    void updateRegisters();
    void updateProgramCounter();
    void updateDecodedInstruction();
    void onProgramCounterChanged();
    McuWrapper mcu;
};

#endif // MAIN_WINDOW_H
