#include "RegisterLineEdit.h"
#include "moc_RegisterLineEdit.cpp"
#include <iostream>

RegisterLineEdit::RegisterLineEdit(QWidget *parent) : 
    QLineEdit(parent) {
    QObject::connect(this, &QLineEdit::editingFinished,
                     this, &RegisterLineEdit::onRegisterChanged);   
}

void RegisterLineEdit::onRegisterChanged() {
    int value = text().toInt(0, 16);
    emit registerChanged(id, value);
}

void RegisterLineEdit::setId(int id) {
    this->id = id;
}