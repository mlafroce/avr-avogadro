#include <QApplication>
#include "MainWindow.h"
#include "mcu_wrapper.h"

extern "C" {
    int run_avogadro_gui(int argc, char *argv[], Mcu* mcu) {
        QApplication app(argc, argv);
        MainWindow mainWindow(0, mcu);
        mainWindow.show();
        return app.exec();
    }
}
