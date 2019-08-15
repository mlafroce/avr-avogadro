#include "RegisterLineEdit.h"
#include "moc_RegisterLineEdit.cpp"
#include <iostream>

RegisterLineEdit::RegisterLineEdit(QWidget *parent) : 
    QLineEdit(parent) {
    QObject::connect(this, &QLineEdit::editingFinished,
                     this, &RegisterLineEdit::registerEdited);   
}

void RegisterLineEdit::registerEdited() {
    std::cout << "editingFinished" << std::endl;
    emit test(id);
}

void RegisterLineEdit::setId(int id) {
    this->id = id;
}