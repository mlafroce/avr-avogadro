#include "MainWindow.h"
#include "RegisterWidget.h"
#include "ui_MainWindow.h"
#include "McuWrapper.h"
#include <cstddef>
#include <QLineEdit>
#include <QFileDialog>

const std::size_t NUM_REGISTERS = 32;
const std::size_t DECODED_INSTRUCTION_BUF = 64;

MainWindow::MainWindow(QMainWindow *parent, void* rustMcu)
 : QMainWindow(parent), mcu(rustMcu) {
    Ui::MainWindow window;
    window.setupUi(this);
    QGroupBox* registerGroupBox = findChild<QGroupBox*>("registerGroupBox");
    findChild<RegisterWidget*>("registerWidget")->setMcu(this->mcu);
    registerGroupBox->setVisible(false);
    this->updateMcuStatus();
    connectEvents();
}

MainWindow::~MainWindow() {}

void MainWindow::mcuStep() {
    this->mcu.step();
    this->updateMcuStatus();
}

void MainWindow::updateMcuStatus() {
    updateProgramCounter();
    updateRegisters();
    updateDecodedInstruction();
}

void MainWindow::updateRegisters() {
    char registers[NUM_REGISTERS];
    this->mcu.getRegisterArray(registers);
    findChild<RegisterWidget*>("registerWidget")->updateRegisters(registers);
}

void MainWindow::updateProgramCounter() {
    QLineEdit* pcEdit = findChild<QLineEdit*>("pcEdit");
    unsigned short pcValue = this->mcu.getProgramCounter();
    QString regText = QString("%1").arg(pcValue, 4, 16, QChar('0'));
    pcEdit->setText(regText);
    QLineEdit* instructionEdit = findChild<QLineEdit*>("instructionEdit");
    unsigned short curInstruction = this->mcu.getCurrentInstruction();
    QString instructionText = QString("%1").arg(curInstruction, 4, 16, QChar('0'));
    instructionEdit->setText(instructionText);
}

void MainWindow::updateDecodedInstruction() {
    char buf[DECODED_INSTRUCTION_BUF];
    this->mcu.displayCurrentInstruction(buf, sizeof(buf));
    findChild<QLabel*>("decodedInstructionLabel")->setText(buf);
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
    if (filename.size() != 0) {
        this->mcu.loadFile(filename.c_str());
        this->updateMcuStatus();
    }    
}

std::string MainWindow::getSelectedFilename() {
    return QFileDialog::getOpenFileName(this,
        tr("Load memory"), "",
        tr("Binary file (*.bin);;All Files (*)")).toStdString();
}
