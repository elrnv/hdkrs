#pragma once

#include <memory>
#include <UT/UT_Interrupt.h>

//class UT_AutoInterrupt;

namespace hdkrs {

struct InterruptChecker {
    std::unique_ptr<UT_AutoInterrupt> progress;
    InterruptChecker(const char * status_message);
    bool check_interrupt();
};

std::unique_ptr<InterruptChecker> new_interrupt_checker(const std::string &message);

} // namespace hdkrs
