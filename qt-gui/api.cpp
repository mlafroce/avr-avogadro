#include <QApplication>
#include "MainWindow.h"
#include "McuWrapper.h"

extern "C" {
    int run_avogadro_gui(int argc, char *argv[], void* mcu) {
        QApplication app(argc, argv);
        MainWindow mainWindow(0, mcu);
        mainWindow.show();
        return app.exec();
    }
}
