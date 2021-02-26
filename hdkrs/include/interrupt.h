#pragma once

#include <UT/UT_Interrupt.h>

namespace hdkrs {

// Interrupt checker.
struct InterruptChecker {
    UT_AutoInterrupt progress;

    InterruptChecker(const char * status_message)
        : progress(status_message) {
    }
};

bool check_interrupt(const void *interrupt) {
    return static_cast<InterruptChecker*>(const_cast<void *>(interrupt))->progress.wasInterrupted();
}

} // namespace interrupt
