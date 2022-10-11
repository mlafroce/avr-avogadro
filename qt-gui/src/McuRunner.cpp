#include "McuRunner.h"
#include "moc_McuRunner.cpp"

#include "McuWrapper.h"

McuRunner::McuRunner(McuWrapper& mcuWrapper) : mcuWrapper(mcuWrapper) {}

void McuRunner::run() {
    this->running.store(true);
    while(this->running.load()) {
        this->mcuWrapper.step();
    }
}

void McuRunner::stop() {
    this->running.store(false);
}
