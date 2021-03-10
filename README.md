# A Rust Houdini Development Kit

This repository introduces an unofficial HDK plugin interface for Rust.

This package contains a collection of functions and types for converting data between Rust and the
C++ interface in the Houdini Development Kit (HDK). This workspace consists of
 - [`hdkrs`](hdkrs): provides functions and types for converting HDK data into Rust and vice versa.
 - [`objio`](objio): an exmaple for using `hdkrs` of a plugin for saving and loading wavefront obj files.
 - [`vtkio`](vtkio): a Houdini plugin for saving and loading [VTK](https://vtk.org) files.

We use the [`cxx`](https://cxx.rs/) crate to facilitate an intuitive interface for C++.


# Building and Installing

## Prerequisites
In general you will need the following tools available on your system:
- [rustc and Cargo](https://www.rust-lang.org/learn/get-started).
- [CMake](https://cmake.org/) for building the C++ part of HDK plugins and `hdkrs` itself.
- [cargo-hdk](https://crates.io/crates/cargo-hdk) subcommand for connecting the Rust build to the
  CMake project (this subcommand should make building the plugin a one-step process). To install,
  simply run
  ```
  > cargo install cargo-hdk
  ```
  in your terminal after you have installed cargo.

You will then need to [setup the Houdini development
environment](https://www.sidefx.com/docs/hdk/_h_d_k__intro__getting_started.html). On Linux and macOS this means sourcing the `houdini_setup` script from the Houdini install directory. On Windows this means opening the "Command Line Tools" program provided with the Houdini install.

## Building and Installing an HDK plugin

First make sure that all the steps in the section above are completed.

Change the current working directory (or `cd`) to the root directory of the plugin you want to build (e.g. `objio` in this repo), and run the following command:
```
> cargo hdk --release
```
which should build the Rust crate, the CMake plugin, link them and install it for your corresponding Houdini version.
For a debug build, run the same command without the `--release` flag.
Note that `cargo-hdk` will place additional files in `hdk/build_*/rust/` to help CMake locate the cargo build.

# Contributing

There are a number of areas that need imporovement:
 - This repo can use more plugin examples.
 - The `hdkrs` crate can be extended to include more useful objects to pass between Rust and the HDK. The choice for what to implement here is an opinionated one, but the general aim of `hdkrs` is to carve out a useful common subset of the HDK interface to share between plugins.
   It is not the aim here to exactly mimic the HDK data structures, however the abstractions we use should capture as much of the available data as possible. For instance, when transferring meshes from the HDK, we should be able to carry all or most of the attributes available.

The above points are merely guidelines. The plan for this crate is not set in stone. I am open to suggestions :-)

# License

This repository is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in
the Work by You, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
