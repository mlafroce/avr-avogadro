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
    void mcuStep() const;
    /**
     * Opens a QFileDialog and gets selected file name
     */
    std::string getSelectedFilename();
    /**
     * Loads raw content of a binary file into program memory
     */
    void loadProgramFile();
    /**
     * Opens online help
     */
    void goToHelpUrl() const;
    /**
     * Connects UI events
     */
    void connectEvents();
    /**
     * Refreshes GUI contents
     */
    void updateMcuStatus() const;
    void updateRegisters() const;
    void updateProgramCounter() const;
    void updateDecodedInstruction() const;
    void updateMemoryBank() const;
    void updateFlags() const;
    /**
     * Fired when program counter line input changes
     */
    void onProgramCounterChanged() const;
    /**
     * Checks file extension
     */
    bool isIhex(const std::string& filename) const;
    McuWrapper mcu;
};

#endif // MAIN_WINDOW_H
