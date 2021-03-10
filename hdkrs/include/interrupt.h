#pragma once

#include <memory>

class UT_AutoInterrupt;

namespace hdkrs {

struct InterruptChecker {
    std::unique_ptr<UT_AutoInterrupt> progress;
    InterruptChecker(const char * status_message);
    bool check_interrupt();
};

std::unique_ptr<InterruptChecker> new_interrupt_checker(std::string message);

} // namespace hdkrs
