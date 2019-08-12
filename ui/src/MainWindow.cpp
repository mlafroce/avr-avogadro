#include "MainWindow.h"
#include "ui_MainWindow.h"
#include "mcu_wrapper.h"
#include <QLineEdit>
#include <QFileDialog>
#include <iostream>

const int NUM_REGISTERS = 32;
const int NAME_BUF_SIZE = sizeof("rxxEdit");

MainWindow::MainWindow(QMainWindow *parent, Mcu* mcu) : QMainWindow(parent), mcu(mcu) {
    Ui::MainWindow greeter;
    greeter.setupUi(this);
    QGroupBox* registerGroupBox = findChild<QGroupBox*>("registerGroupBox");
    registerGroupBox->setVisible(false);
    updateRegisters();
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
    QAction* loadFileMenuAction = findChild<QAction*>("loadFileMenuAction");
    QObject::connect(buttonGreet, &QPushButton::clicked,
                     this, &MainWindow::mcuStep);
    QObject::connect(loadFileMenuAction, &QAction::triggered,
                     this, &MainWindow::loadFile);
}

void MainWindow::loadFile() {
    std::string filename = getSelectedFilename();
    mcu_load_file(this->mcu, filename.c_str());
}

std::string MainWindow::getSelectedFilename() {
    return QFileDialog::getOpenFileName(this,
        tr("Load memory"), "",
        tr("Binary file (*.bin);;All Files (*)")).toStdString();
}
