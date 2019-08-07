#ifndef MAIN_WINDOW_H
#define MAIN_WINDOW_H

#include <QWidget>
#include "mcu_wrapper.h"

class MainWindow : public QWidget {
public:
    explicit MainWindow(QWidget *parent, Mcu* mcu);
    virtual ~MainWindow();
private:
    void mcuStep();
    void connectEvents();
    void updateRegisters();
    Mcu* mcu;
};

#endif // MAIN_WINDOW_H
