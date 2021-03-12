# A fast Obj importer and exporter for Houdini

This crate serves as an example for how one may use `hdkrs` to build a plugin for Houdini.

The HDK plugin code is located in the [`hdk`](hdk) directory and builds using
CMake after the Rust crate is built.

The load and save functions are registered in `hdk/src/GEO_ObjIO.cpp`.

The Rust code is located in the `src` directory as usual. It defines
functions to (eventually) call into the [`obj`](https://github.com/kvark/obj)
crate to load and save the `.obj` files. The [`gut`](https://crates.io/crates/gut) crate provides convenient mesh types to interface with obj files, although it is not strictly necessary here.

## Building

See the "Building and Installing" section in the [upper level README](../README.md) for instructions.

# Highlights

Although this plugin doesn't generate exactly the same imported geometry (using attributes instead of groups), it can already be over 2x faster than the built-in obj loader in Houdini when tested on Linux.