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
    void loadFile();
    std::string getSelectedFilename();    
    void connectEvents();
    void updateRegisters();
    McuWrapper mcu;
};

#endif // MAIN_WINDOW_H
