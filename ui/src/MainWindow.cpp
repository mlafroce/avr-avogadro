#include "MainWindow.h"
#include "RegisterWidget.h"
#include "ui_MainWindow.h"
#include "McuWrapper.h"
#include <QLineEdit>
#include <QFileDialog>

const int NUM_REGISTERS = 32;

MainWindow::MainWindow(QMainWindow *parent, void* rustMcu)
 : QMainWindow(parent), mcu(rustMcu) {
    Ui::MainWindow window;
    window.setupUi(this);
    QGroupBox* registerGroupBox = findChild<QGroupBox*>("registerGroupBox");
    findChild<RegisterWidget*>("registerWidget")->setMcu(this->mcu);
    registerGroupBox->setVisible(false);
    updateRegisters();
    connectEvents();
}

MainWindow::~MainWindow() {}

void MainWindow::mcuStep() {
    this->mcu.step();
    updateRegisters();
    updateProgramCounter();
}

void MainWindow::updateRegisters() {
    char registers[NUM_REGISTERS];
    this->mcu.getRegisterArray(registers);
    findChild<RegisterWidget*>("registerWidget")->updateRegisters(registers);
}

void MainWindow::updateProgramCounter() {
    QLineEdit* pcEdit = findChild<QLineEdit*>("pcEdit");
    short pcValue = this->mcu.getProgramCounter();
    QString regText = QString("%1").arg(pcValue, 0, 16);
    pcEdit->setText(regText);
}

void MainWindow::onProgramCounterChanged() {
    QLineEdit* pcEdit = findChild<QLineEdit*>("pcEdit");
    int value = pcEdit->text().toInt(0, 16);
    this->mcu.setProgramCounter(value);
}

void MainWindow::connectEvents() {
    QPushButton* buttonGreet = findChild<QPushButton*>("stepButton");
    QAction* loadFileMenuAction = findChild<QAction*>("loadFileMenuAction");
    QLineEdit* pcEdit = findChild<QLineEdit*>("pcEdit");
    QObject::connect(buttonGreet, &QPushButton::clicked,
                     this, &MainWindow::mcuStep);
    QObject::connect(loadFileMenuAction, &QAction::triggered,
                     this, &MainWindow::loadFile);
    QObject::connect(pcEdit, &QLineEdit::editingFinished,
                     this, &MainWindow::onProgramCounterChanged);
}

void MainWindow::loadFile() {
    std::string filename = getSelectedFilename();
    this->mcu.loadFile(filename.c_str());
}

std::string MainWindow::getSelectedFilename() {
    return QFileDialog::getOpenFileName(this,
        tr("Load memory"), "",
        tr("Binary file (*.bin);;All Files (*)")).toStdString();
}
