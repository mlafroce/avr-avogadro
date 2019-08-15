#ifndef REGISTER_WIDGET_H
#define REGISTER_WIDGET_H

#include <QWidget>

class RegisterWidget : public QWidget {
public:
    explicit RegisterWidget(QWidget *parent);
    virtual ~RegisterWidget();
    void updateRegisters(char* registers);
    void testSlot(int id);
private:
    void connectEvents();
};

#endif // REGISTER_WIDGET_H
