#include "MainWindow.h"
#include "RegisterWidget.h"
#include "ui_MainWindow.h"
#include "McuWrapper.h"
#include "qhexedit.h"
#include <cstddef>
#include <QByteArray>
#include <QLineEdit>
#include <QFileDialog>
#include <QDesktopServices>

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

void MainWindow::mcuStep() const {
    this->mcu.step();
    this->updateMcuStatus();
}

void MainWindow::updateMcuStatus() const {
    updateProgramCounter();
    updateRegisters();
    updateDecodedInstruction();
    updateFlags();
}

void MainWindow::updateMemoryBank() const {
    std::vector<char> buf;
    this->mcu.getMemoryBank(buf);
    QByteArray bytes(buf.data(), buf.size());
    findChild<QHexEdit*>("hexEdit")->setData(bytes);
}

void MainWindow::updateRegisters() const {
    unsigned char registers[NUM_REGISTERS];
    this->mcu.getRegisterArray(registers);
    findChild<RegisterWidget*>("registerWidget")->updateRegisters(registers);
}

void MainWindow::updateProgramCounter() const {
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

void MainWindow::updateDecodedInstruction() const {
    char buf[DECODED_INSTRUCTION_BUF];
    this->mcu.displayCurrentInstruction(buf, sizeof(buf));
    findChild<QLabel*>("decodedInstructionLabel")->setText(buf);
}

void MainWindow::updateFlags() const {
    unsigned char flags = this->mcu.getFlags();
    findChild<QCheckBox*>("iCheckBox")->setChecked((flags & 0x80) != 0);
    findChild<QCheckBox*>("tCheckBox")->setChecked((flags & 0x40) != 0);
    findChild<QCheckBox*>("hCheckBox")->setChecked((flags & 0x20) != 0);
    findChild<QCheckBox*>("sCheckBox")->setChecked((flags & 0x10) != 0);
    findChild<QCheckBox*>("vCheckBox")->setChecked((flags & 0x08) != 0);
    findChild<QCheckBox*>("nCheckBox")->setChecked((flags & 0x04) != 0);
    findChild<QCheckBox*>("zCheckBox")->setChecked((flags & 0x02) != 0);
    findChild<QCheckBox*>("cCheckBox")->setChecked((flags & 0x01) != 0);
}

void MainWindow::onProgramCounterChanged() const {
    NumericEdit* pcEdit = findChild<NumericEdit*>("pcEdit");
    short value = pcEdit->getWord();
    this->mcu.setProgramCounter(value);
}

void MainWindow::connectEvents() {
    QPushButton* buttonGreet = findChild<QPushButton*>("stepButton");
    QAction* loadProgamFileMenuAction = findChild<QAction*>("loadProgamFileMenuAction");
    QAction* gettingStartedMenuAction = findChild<QAction*>("gettingStartedMenuAction");
    QLineEdit* pcEdit = findChild<QLineEdit*>("pcEdit");
    QObject::connect(buttonGreet, &QPushButton::clicked,
                     this, &MainWindow::mcuStep);
    QObject::connect(loadProgamFileMenuAction, &QAction::triggered,
                     this, &MainWindow::loadProgramFile);
    QObject::connect(gettingStartedMenuAction, &QAction::triggered,
                     this, &MainWindow::goToHelpUrl);
    QObject::connect(pcEdit, &NumericEdit::editingFinished,
                     this, &MainWindow::onProgramCounterChanged);
}

void MainWindow::loadProgramFile() {
    std::string filename = getSelectedFilename();
    if (filename.size() != 0) {
        if (isIhex(filename)) {
            this->mcu.loadIhexFile(filename.c_str());
        } else {
            this->mcu.loadBinFile(filename.c_str(), true);
        }
        this->updateMcuStatus();
    }
    this->updateMemoryBank();
}

void MainWindow::goToHelpUrl() const {
    QUrl helpUrl("https://mlafroce.github.io/avr-avogadro/getting-started");
    QDesktopServices::openUrl(helpUrl);
}

bool MainWindow::isIhex(const std::string& filename) const {
    if (filename.length() >= 4) {
        return (0 == filename.compare (filename.length() - 4, 4, ".hex"));
    } else {
        return false;
    }
}

std::string MainWindow::getSelectedFilename() {
    return QFileDialog::getOpenFileName(this,
        tr("Load memory"), "",
        tr("Binary file (*.bin);;IHex file (*.hex);;All Files (*)")).toStdString();
}
