#ifndef REGISTER_LINE_EDIT_H
#define REGISTER_LINE_EDIT_H

#include <QLineEdit>

class RegisterLineEdit : public QLineEdit {
    Q_OBJECT
public:
    explicit RegisterLineEdit(QWidget *parent);
    void onRegisterChanged();
    void setId(int id);
signals:
    void registerChanged(int id, int value);
private:
    int id;
};

#endif // REGISTER_LINE_EDIT_H
