#include "RegisterWidget.h"
#include "RegisterLineEdit.h"
#include "ui_RegisterWidget.h"
#include <QLineEdit>
#include <cstdio>

const int NUM_REGISTERS = 32;
const int NAME_BUF_SIZE = sizeof("rxxEdit") + 1;

RegisterWidget::RegisterWidget(QWidget *parent) : 
        QWidget(parent), mcu(0) {
    Ui::RegisterWidget registerWidgetUi;
    registerWidgetUi.setupUi(this);
    connectEvents();
}

RegisterWidget::~RegisterWidget() {}

void RegisterWidget::onRegisterChanged(int id, int value) {
    this->mcu.setRegister(id, value);
}

void RegisterWidget::setMcu(const McuWrapper& mcu) {
    this->mcu = mcu;
}

void RegisterWidget::connectEvents() {
    char nameBuf[NAME_BUF_SIZE];
    for (int regNum = 0; regNum < NUM_REGISTERS; ++regNum) {
        snprintf(nameBuf, sizeof(nameBuf), "r%dEdit", regNum);
        RegisterLineEdit* lineEdit = findChild<RegisterLineEdit*>(nameBuf);
        lineEdit->setId(regNum);
        QObject::connect(lineEdit, &RegisterLineEdit::registerChanged,
                         this, &RegisterWidget::onRegisterChanged);
    }
}

void RegisterWidget::updateRegisters(unsigned char* registers) {
    char nameBuf[NAME_BUF_SIZE];
    for (int regNum = 0; regNum < NUM_REGISTERS; ++regNum) {
        snprintf(nameBuf, sizeof(nameBuf), "r%dEdit", regNum);
        QLineEdit* edit = findChild<QLineEdit*>(nameBuf);
        QString regText = QString("%1").arg(registers[regNum], 0, 16);
        edit->setText(regText);
    }
}
