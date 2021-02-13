# hdkrs

An HDK plugin library for Rust

This package contains a collection of functions and types for converting data between Rust and the
Houdini Development Kit (HDK).

# Examples

`obj` is a simple example for using hdkrs to inteface between Rust and the HDK. Here, an HDK plugin
is registered for reading and writing obj files.


## Structure

There package contains:
    1. A C FFI for passing data into Rust defined in `cffi.rs` and automatically translated into C using
       `cbindgen`.
    2. A collection of C++ utilities to process data from the HDK and convert it into types defined
       in the C FFI. These are provided as a header-only implementation, which avoids having an
       additional C++ compilation step. These utilities augment the pure C API.

Note that the additional C++ utilities are namespaced with hdkrs and their corresponding categories.
The pure C API uses prefix conventions for resolving name collisions.
