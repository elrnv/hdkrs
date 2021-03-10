#include <hdkrs/src/lib.rs.h>
#include <rust/cxx.h>
#include "mesh.h"
#include "interrupt.h"

namespace hdkrs {
    /// Utility function for casting a Box of one type to a Box of another.
    /// This is only safe when the the two types have identical memory layouts and were allocated with the same Rust allocator.
    /// This function is useful for casting between transparent newtypes to other Rust types on the other side of the cxx bridge.
    template<typename To, typename From>
    rust::box<To> cast_box(rust::box<From> &&from) {
        return rust::box<To>::from_raw(reinterpret_cast<To*>(from.into_raw()));
    }
}