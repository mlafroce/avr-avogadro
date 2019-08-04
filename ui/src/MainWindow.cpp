#include "MainWindow.h"
#include "ui_MainWindow.h"
#include "mcu_wrapper.h"

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
}

void MainWindow::connectEvents() {
    QPushButton* buttonGreet = findChild<QPushButton*>("stepButton");
    QObject::connect(buttonGreet, &QPushButton::clicked,
                     this, &MainWindow::mcuStep);
}
