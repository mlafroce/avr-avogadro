#ifndef REGISTER_WIDGET_H
#define REGISTER_WIDGET_H

#include <QWidget>

#include "mcu_wrapper.h"

class RegisterWidget : public QWidget {
public:
    explicit RegisterWidget(QWidget *parent);
    virtual ~RegisterWidget();
    void updateRegisters(char* registers);
    void onRegisterChanged(int id, int value);
    void setMcu(Mcu* mcu);
private:
    void connectEvents();
    Mcu* mcu;
};

#endif // REGISTER_WIDGET_H
