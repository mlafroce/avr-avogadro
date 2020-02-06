#ifndef NUMERIC_EDIT_H
#define NUMERIC_EDIT_H

#include <QLineEdit>

class NumericEdit : public QLineEdit {
public:
    explicit NumericEdit(QWidget *parent);
    virtual ~NumericEdit();
    short getWord();
    void setWord(unsigned short word);
};

#endif // NUMERIC_EDIT_H
