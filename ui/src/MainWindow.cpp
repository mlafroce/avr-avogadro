#include "MainWindow.h"
#include "ui_MainWindow.h"
#include "mcu_wrapper.h"
#include <QLineEdit>

const int NUM_REGISTERS = 32;
const int NAME_BUF_SIZE = sizeof("rxxEdit");

MainWindow::MainWindow(QWidget *parent, Mcu* mcu) : QWidget(parent), mcu(mcu) {
    Ui::MainWindow greeter;
    greeter.setupUi(this);
    QGroupBox* registerGroupBox = findChild<QGroupBox*>("registerGroupBox");
    registerGroupBox->setVisible(false);
    connectEvents();
}

MainWindow::~MainWindow() {}

void MainWindow::mcuStep() {
    mcu_step(this->mcu);
    updateRegisters();
}

void MainWindow::updateRegisters() {
    char registers[NUM_REGISTERS];
    char nameBuf[NAME_BUF_SIZE];
    mcu_get_register_array(this->mcu, registers);
    for (int regNum = 0; regNum < NUM_REGISTERS; ++regNum) {
        snprintf(nameBuf, sizeof(nameBuf), "r%dEdit", regNum);
        QLineEdit* edit = findChild<QLineEdit*>(nameBuf);
        QString regText = QString("%1").arg(registers[regNum], 0, 16);
        edit->setText(regText);
    }
}

void MainWindow::connectEvents() {
    QPushButton* buttonGreet = findChild<QPushButton*>("stepButton");
    QObject::connect(buttonGreet, &QPushButton::clicked,
                     this, &MainWindow::mcuStep);
}
