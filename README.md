# A Rust Houdini Development Kit

This repository introduces an unofficial HDK plugin interface for Rust.

This package contains a collection of functions and types for converting data between Rust and the
C++ interface in the Houdini Development Kit (HDK). This workspace consists of
 - [`hdkrs`](hdkrs): provides functions and types for converting HDK data into Rust and vice versa.
 - [`objio`](objio): an exmaple for using `hdkrs` of a plugin for saving and loading wavefront obj files.
 - [`vtkio`](vtkio): a Houdini plugin for saving and loading [VTK](https://vtk.org) files.

We use the [`cxx`](https://cxx.rs/) crate to facilitate an intuitive interface for C++.

For more details see the [`hdkrs` `README.md`](hdkrs/README.md).


# License

This repository is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in
the Work by You, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
