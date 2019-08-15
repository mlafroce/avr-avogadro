#include "MainWindow.h"
#include "RegisterWidget.h"
#include "ui_MainWindow.h"
#include "mcu_wrapper.h"
#include <QLineEdit>
#include <QFileDialog>
#include <iostream>

const int NUM_REGISTERS = 32;

MainWindow::MainWindow(QMainWindow *parent, Mcu* mcu) : QMainWindow(parent), mcu(mcu) {
    Ui::MainWindow window;
    window.setupUi(this);
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
    mcu_get_register_array(this->mcu, registers);
    findChild<RegisterWidget*>("registerWidget")->updateRegisters(registers);
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
