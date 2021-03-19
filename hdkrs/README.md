# hdkrs

An unofficial Houdini Development Kit plugin interface for Rust.

This package aims to carve out a useful Rust API for interfacing with the HDK.

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


# Build Structure

This library lays out a structure for building HDK plugins with Rust.
Since building Houdini and linking to Rust and C++ is rather involved, the build process could get complicated. Below we will outline how we build a plugin when using `hdkrs` and some of the problems encountered.

Building against the HDK is involved, so we opt to leverage the CMake build process, which is maintained by SideFX themselves. This saves a ton of maintenance for this library.

We also rely heavily on cargo to manage dependencies and try to maintain a familiar development environment for Rustaceans.

## The `hdkrs` dependency

`hdkrs` itself is a Rust crate that uses HDK headers. It is intended to be included as a crate through cargo and thus intended to be statically linked to end point crates that implement the plugins themselves. Building with HDK headers involves careful compiler setup so this is done through CMake in [`hdkrs/CMakeLists.txt`](hdkrs/CMakeLists.txt) which is called by the build script in [`hdkrs/build.rs`](hdkrs/build.rs). `build.rs` also generates the `cxx` bridge and builds the generated `cxx` bridge sources along with the additional C++ code located in `hdkrs/src` and `hdkrs/include`. This bridge is then linked to the Rust library.

## The Plugin

A downstream plugin includes `hdkrs` as a cargo dependency
but it will need to sport up to two of its own `CMakeLists.txt` files to properly build and link all the involved libraries. Since `hdkrs` uses `cxx` to bridge C++ and Rust code, to make the most out of `hdkrs` we recommend relying on `cxx` and `cxx-build` in plugin code as well to make use of the shared helper types and functions in `hdkrs`.

The plugin will be structured around a regular Rust crate, so to start, initialize a Rust library crate.

0. Add the following dependencies to your Cargo.toml manifest:
```toml
[dependencies]
hdkrs = "0.1"
cxx = "1.0"

[build-dependencies]
cxx-build = "1.0"
cmake = { git = "https://github.com/elrnv/cmake-rs.git", version = "0.1" }
```
The `cmake` dependencies above references a fork, which exposes configuration options that allows you to feed the output of `cxx_build::bridge` into `cmake`. This will save on a lot of manual configuration. (Otherwise we may either need to configure compiling with the HDK or building the bridge manually).
Changes to `cmake` are upstreamed in https://github.com/alexcrichton/cmake-rs/pull/112.

1. Create a `CMakeLists.txt` file for building the `cxx` bridge and linking to the HDK.
   The main task of this `CMakeLists.txt` can be summarized as follows:
   ```cmake
   # CMAKE_PREFIX_PATH must contain the path to the toolkit/cmake subdirectory of
   # the Houdini installation. See the "Compiling with CMake" section of the HDK
   # documentation for more details, which describes several options for
   # specifying this path.
   list( APPEND CMAKE_PREFIX_PATH "$ENV{HFS}/toolkit/cmake" )

   # Locate Houdini's libraries and header files.
   # This registers an imported library target named 'Houdini'.
   find_package( Houdini REQUIRED )

   # Add a library and its source files. Inlude all cxxbridge sources here.
   add_library( cxxbridge STATIC "${CMAKE_BINARY_DIR}/../cxxbridge/sources/<crate-name>/src/lib.rs.cc")

   # Link against the Houdini libraries, and add required include directories and
   # compile definitions.
   target_link_libraries( cxxbridge Houdini )
   ```
   where `<crate-name>` should be replaced with the name of your crate. You can use any name for this library target in place of `cxxbridge`.
2. Add a `build.rs` script that will call `cxx_build` to generate the bridge sources, then build it with `cmake` and instruct rust to link to the generated lib as follows:
   ```rust
   // Generate the bridge
   let build = cxx_build::bridge("src/lib.rs");

   // Build with CMake.
   cmake::Config::new(".")
       .no_build_target(true) // Skip the install step
       .init_c_cfg(build.clone())
       .init_cxx_cfg(build)
       .build();

   // Get the output directory where we will find the generated lib.
   let out_dir = std::env::var("OUT_DIR").unwrap();

   println!("cargo:rerun-if-changed=src/lib.rs");
   println!("cargo:rustc-link-lib=static=cxxbridge");
   println!("cargo:rustc-link-search=native={}/build", out_dir);
   ```
   > ### Note
   > In our plugins, in `CMakeLists.txt` we used 
   > ```cmake
   > houdini_configure_target( ${library_name} INSTDIR ${CMAKE_INSTALL_PREFIX} LIB_PREFIX lib)
   > ```
   > to install the bridge right in the output directory, which would change the last line of the `build.rs` script to
   > ```rust
   > println!("cargo:rustc-link-search=native={}", out_dir);
   > ```
   > This is optional.
3. Create an `hdk` directory where you will create a normal Houdini HDK C++ plugin, which can use CMake's `find_package` to get the Rust static lib and include headers (i.e. this will have its own `CMakeLists.txt`). For details of how to build a Houdini plugin with CMake, please refer to the [HDK docs](https://www.sidefx.com/docs/hdk/_h_d_k__intro__compiling.html#HDK_Intro_Compiling_CMake).


# Contributing

There are a number of areas that need imporovement:
 - This repo can use more plugin examples.
 - The `hdkrs` crate can be extended to include more useful objects to pass between Rust and the HDK. The choice for what to implement here is an opinionated one, but the general aim of `hdkrs` is to carve out a useful common subset of the HDK interface to share between plugins.
   It is not the aim here to exactly mimic the HDK data structures, however the abstractions we use should capture as much of the available data as possible. For instance, when transferring meshes from the HDK, we should be able to carry all or most of the attributes available.
 - It would be ideal to simplify the build system requirements for downstream crates. Currently  downstream crates require `cmake`, `cxx`, `cxx-build` and `hdkrs` dependencies as well as potentially two `CMakeLists.txt` files to successfully build and link to all the necessary dependencies.

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
