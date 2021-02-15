#pragma once

#include <hdkrs.h>
#include <cassert>

namespace hdkrs {

// An owned pointer to a preallocated resource that automatically frees it upon destruction.
template<typename T>
class OwnedPtr {
public:
    OwnedPtr(OwnedPtr && other)
        : _ptr(other._ptr)
    {
        other._ptr = nullptr;
    }
    OwnedPtr(T* ptr) : _ptr(ptr) { }
    ~OwnedPtr(); // must be specialized for each T 

    OwnedPtr& operator=(OwnedPtr && other) {
        if (this != &other) {
            this->~OwnedPtr();

            _ptr = other._ptr;
            other._ptr = nullptr;
        }
        return *this;
    }

    // Remove copy
    OwnedPtr(const OwnedPtr & other) = delete;
    OwnedPtr& operator=(const OwnedPtr & other) = delete;

    operator bool() {
        return _ptr != nullptr;
    }

    T& operator *() {
        assert(_ptr);
        return *_ptr;
    }

    T* operator->() {
        assert(_ptr);
        return _ptr;
    }

    T* get() {
        return _ptr;
    }

    // Release the ownership of the stored pointer.
    T* release() {
        T* ptr = _ptr;
        _ptr = nullptr;
        return ptr;
    }

private:
    T* _ptr;
};

} // namespace hdkrs

