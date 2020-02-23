#ifndef MAIN_WINDOW_H
#define MAIN_WINDOW_H

#include <QMainWindow>
#include "McuWrapper.h"

class MainWindow : public QMainWindow {
public:
    explicit MainWindow(QMainWindow *parent, void* mcu);
    virtual ~MainWindow();
private:
    /**
     * Executes a single isntruction
     */
    void mcuStep();
    /**
     * Opens a QFileDialog and gets selected file name
     */
    std::string getSelectedFilename();
    /**
     * Loads raw content of a binary file
     */
    void loadFile();
    /**
     * Opens online help
     */
    void goToHelpUrl();
    /**
     * Connects UI events
     */
    void connectEvents();
    /**
     * Refreshes GUI contents
     */
    void updateMcuStatus();
    void updateRegisters();
    void updateProgramCounter();
    void updateDecodedInstruction();
    void updateMemoryBank();
    void updateFlags();
    /**
     * Fired when program counter line input changes
     */
    void onProgramCounterChanged();
    McuWrapper mcu;
};

#endif // MAIN_WINDOW_H
