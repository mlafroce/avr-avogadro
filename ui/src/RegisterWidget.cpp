#include "RegisterWidget.h"
#include "RegisterLineEdit.h"
#include "ui_RegisterWidget.h"
#include <QLineEdit>
#include <iostream>

const int NUM_REGISTERS = 32;
const int NAME_BUF_SIZE = sizeof("rxxEdit");

RegisterWidget::RegisterWidget(QWidget *parent) : 
        QWidget(parent) {
    Ui::RegisterWidget registerWidgetUi;
    registerWidgetUi.setupUi(this);
    connectEvents();
}

RegisterWidget::~RegisterWidget() {}

void RegisterWidget::testSlot(int id) {
    std::cout << "Test slot: " << id << std::endl;
}

void RegisterWidget::connectEvents() {
    RegisterLineEdit* testEdit = findChild<RegisterLineEdit*>("r0Edit");
    QObject::connect(testEdit, &RegisterLineEdit::test,
                     this, &RegisterWidget::testSlot);
}

void RegisterWidget::updateRegisters(char* registers) {
    char nameBuf[NAME_BUF_SIZE];
    for (int regNum = 0; regNum < NUM_REGISTERS; ++regNum) {
        snprintf(nameBuf, sizeof(nameBuf), "r%dEdit", regNum);
        QLineEdit* edit = findChild<QLineEdit*>(nameBuf);
        QString regText = QString("%1").arg(registers[regNum], 0, 16);
        edit->setText(regText);
    }
}
