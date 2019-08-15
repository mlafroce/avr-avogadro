#ifndef MAIN_WINDOW_H
#define MAIN_WINDOW_H

#include <QMainWindow>
#include "mcu_wrapper.h"

class MainWindow : public QMainWindow {
public:
    explicit MainWindow(QMainWindow *parent, Mcu* mcu);
    virtual ~MainWindow();
private:
    void mcuStep();
    void loadFile();
    std::string getSelectedFilename();    
    void connectEvents();
    void updateRegisters();
    Mcu* mcu;
};

#endif // MAIN_WINDOW_H
