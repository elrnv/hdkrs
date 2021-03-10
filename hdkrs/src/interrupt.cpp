#include "interrupt.h"

#include <memory>
#include <UT/UT_Interrupt.h>

using namespace hdkrs;


InterruptChecker::InterruptChecker(const char * status_message)
    : progress(std::make_unique<UT_AutoInterrupt>(status_message)) {
}

bool InterruptChecker::check_interrupt() {
    return this->progress->wasInterrupted();
}

std::unique_ptr<InterruptChecker> new_interrupt_checker(const std::string &message) {
    return std::make_unique<InterruptChecker>(message.c_str());
}
