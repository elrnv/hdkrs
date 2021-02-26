# hdkrs

This package aims to carve out a useful Rust API for interfacing with the Houdini Development Kit (HDK).

## Structure

Here we offer a set of headers for interfacing with the HDK (located in the
[`include`](include) directory) as well as a Rust lib with a C++ bridge built
using [`cxx`](https://cxx.rs). This crate relies on cargo for building Rust
code, but also exposes a CMake config file, `hdkrsConfig.cmake` to help
downstream crates find the necessary C++ headers also exposed by this library
to interface with Rust code.

The [`src`](src) directory contains the Rust code and cxx bridge for interfacing with C++.
We use [`cxx-build`](https://docs.rs/cxx-build/1.0.32/cxx_build/) to build
the bridge, which is statically linked against the Rust library.

It is recommended to use the
[`cargo-hdk`](https://github.com/elrnv/cargo-hdk) command-line tool to build
your Rust HDK plugin when using this crate as a dependency. For an example of
how to build an HDK plugin in Rust, take a look at the [`objio`](../objio)
package.

# License

This repository is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in the
Work by You, as defined in the [Apache-2.0 license](LICENSE-APACHE), shall be dual licensed as
above, without any additional terms or conditions.