#ifndef REGISTER_LINE_EDIT_H
#define REGISTER_LINE_EDIT_H

#include <QLineEdit>

class RegisterLineEdit : public QLineEdit {
    Q_OBJECT
public:
    explicit RegisterLineEdit(QWidget *parent);
    virtual void registerEdited();
    void setId(int id);
signals:
    void test(int id);
private:
    int id;
};

#endif // REGISTER_LINE_EDIT_H
