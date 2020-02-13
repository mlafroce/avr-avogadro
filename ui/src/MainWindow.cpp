#include "MainWindow.h"
#include "RegisterWidget.h"
#include "ui_MainWindow.h"
#include "McuWrapper.h"
#include "qhexedit.h"
#include <cstddef>
#include <QByteArray>
#include <QLineEdit>
#include <QFileDialog>

const std::size_t NUM_REGISTERS = 32;
const std::size_t DECODED_INSTRUCTION_BUF = 64;

MainWindow::MainWindow(QMainWindow *parent, void* rustMcu)
 : QMainWindow(parent), mcu(rustMcu) {
    Ui::MainWindow window;
    window.setupUi(this);
    findChild<RegisterWidget*>("registerWidget")->setMcu(this->mcu);
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

void MainWindow::updateMemoryBank() {
    std::vector<char> buf;
    this->mcu.getMemoryBank(buf);
    QByteArray bytes(buf.data(), buf.size());
    findChild<QHexEdit*>("hexEdit")->setData(bytes);
}

void MainWindow::updateRegisters() {
    unsigned char registers[NUM_REGISTERS];
    this->mcu.getRegisterArray(registers);
    findChild<RegisterWidget*>("registerWidget")->updateRegisters(registers);
}

void MainWindow::updateProgramCounter() {
    NumericEdit* pcEdit = findChild<NumericEdit*>("pcEdit");
    NumericEdit* instructionEdit = findChild<NumericEdit*>("instructionEdit");
    NumericEdit* stackPointerEdit = findChild<NumericEdit*>("stackPointerEdit");
    unsigned short pcValue = this->mcu.getProgramCounter();
    pcEdit->setWord(pcValue);
    unsigned short curInstruction = this->mcu.getCurrentInstruction();
    instructionEdit->setWord(curInstruction);
    unsigned short stackPointer = this->mcu.getStackPointer();
    stackPointerEdit->setWord(stackPointer);
}

void MainWindow::updateDecodedInstruction() {
    char buf[DECODED_INSTRUCTION_BUF];
    this->mcu.displayCurrentInstruction(buf, sizeof(buf));
    findChild<QLabel*>("decodedInstructionLabel")->setText(buf);
}

void MainWindow::onProgramCounterChanged() {
    NumericEdit* pcEdit = findChild<NumericEdit*>("pcEdit");
    short value = pcEdit->getWord();
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
    QObject::connect(pcEdit, &NumericEdit::editingFinished,
                     this, &MainWindow::onProgramCounterChanged);
}

void MainWindow::loadFile() {
    std::string filename = getSelectedFilename();
    if (filename.size() != 0) {
        this->mcu.loadFile(filename.c_str());
        this->updateMcuStatus();
    }
    this->updateMemoryBank();
}

std::string MainWindow::getSelectedFilename() {
    return QFileDialog::getOpenFileName(this,
        tr("Load memory"), "",
        tr("Binary file (*.bin);;All Files (*)")).toStdString();
}
