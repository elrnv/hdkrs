# A Rust Houdini Development Kit

This repository introduces an unofficial HDK plugin interface for Rust.

This package contains a collection of functions and types for converting data between Rust and the
C++ interface in the Houdini Development Kit (HDK). This workspace consists of
 - [`hdkrs`](hdkrs): provides functions and types for converting HDK data into Rust and vice versa.
 - [`objio`](objio): an exmaple for using `hdkrs` of a plugin for saving and loading wavefront obj files.
 - [`vtkio`](vtkio): a Houdini plugin for saving and loading [VTK](https://vtk.org) files.

We use the [`cxx`](https://cxx.rs/) crate to facilitate an intuitive interface for C++.

For more details see the [`hdkrs` `README.md`](hdkrs/README.md).


# Note for Apple Silicon builds

Note that this section applies to the brand new Houdini 19.0.563 apple silicon daily builds, and may become
quickly obsolete.
To build plugins for the new Apple silicon Houdini builds, we must first tweak the cmake script.
After initializing the Houdini environment variables:
  - Open $HFS/toolkit/cmake/HoudiniConfig.cmake in an editor with administrative priveleges.
  - Remove `MBSD_INTEL` from `_houdini_defines` list on line 30, and change `AMD64` to `ARM64`.
  - Add `set(CMAKE_OSX_ARCHITECTURES arm64)` to the script (e.g. before `_houdini_compile_options`
    is defined).
This tweak has already been reported on the [SideFX
forums](https://www.sidefx.com/forum/topic/83559/?page=1#post-360626), so it may not be needed for
long.


# License

This repository is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in
the Work by You, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
