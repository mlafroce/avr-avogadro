#include "NumericEdit.h"
#include <QString>

NumericEdit::NumericEdit(QWidget *parent) : QLineEdit(parent) {
    setInputMask("hhhh");
    setMaxLength(4);
}

void NumericEdit::setWord(unsigned short word) {
    QString text = QString("%1").arg(word, 4, 16, QChar('0'));
    setText(text);
}

short NumericEdit::getWord() {
    return text().toInt(0, 16);
}

NumericEdit::~NumericEdit() {}
