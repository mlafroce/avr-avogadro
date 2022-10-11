#ifndef MCU_RUNNER_H
#define MCU_RUNNER_H

#include <QThread>
#include <atomic>

class McuWrapper;

class McuRunner : public QThread {
Q_OBJECT
public:

    McuRunner(McuWrapper& mcuWrapper);
    void run() override;
    void stop();

signals:
    void runnerFinished();
private:
    McuWrapper& mcuWrapper;
    std::atomic<bool> running = false;
};

#endif // MCU_RUNNER_H