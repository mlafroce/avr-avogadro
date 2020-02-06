#ifndef REGISTER_WIDGET_H
#define REGISTER_WIDGET_H

#include <QWidget>

#include "McuWrapper.h"

class RegisterWidget : public QWidget {
public:
    explicit RegisterWidget(QWidget *parent);
    virtual ~RegisterWidget();
    void updateRegisters(unsigned char* registers);
    void onRegisterChanged(int id, int value);
    void setMcu(const McuWrapper& mcu);
private:
    void connectEvents();
    McuWrapper mcu;
};

#endif // REGISTER_WIDGET_H
