use std::path::{Path, PathBuf};
use std::{env, fs};

use cxx_build::CFG;
use glob::glob;

fn main() {
    // Copy HDK API C headers from source to target directory

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let include_target = out_dir.join("include");
    let header_target = include_target.join("hdkrs");
    if !header_target.exists() {
        fs::create_dir_all(&header_target).unwrap_or_else(|_| {
            panic!(
                "Failed to create target directory for header files: {:?}",
                header_target
            )
        });
    }

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    for entry in glob(&format!("{}/include/*.h", crate_dir)).expect("Failed to find headers.") {
        match entry {
            Ok(src) => {
                let header = src.file_name().unwrap();
                let dst = header_target.join(Path::new(&header));
                println!("copying {:?} to {:?}", src, dst);
                fs::copy(&src, &dst)
                    .unwrap_or_else(|_| panic!("Failed to copy header {:?}", header));
            }
            Err(e) => println!("{:?}", e),
        }
    }

    // Copy CMake config file to "$OUT_DIR/cmake" to make it easier to find by other CMake scripts.
    // Note that the other cmake script still needs to find the correct OUT_DIR.

    let cmake_target = out_dir.join("cmake");
    if !cmake_target.exists() {
        fs::create_dir(&cmake_target).unwrap_or_else(|_| {
            panic!(
                "Failed to create target directory for cmake config files: {:?}",
                cmake_target
            )
        });
    }

    let cmake_config_file = PathBuf::from("hdkrsConfig.cmake");
    let dst = cmake_target.join(&cmake_config_file);
    let src = PathBuf::from(crate_dir).join(&cmake_config_file);
    fs::copy(&src, &dst)
        .unwrap_or_else(|_| panic!("Failed to copy cmake config {:?}", cmake_config_file));

    // Build the cxx bridge

    // Re-export the headers in the cxx bridge target.
    CFG.exported_header_dirs.push(&include_target);

    let build = cxx_build::bridge("src/lib.rs");

    cmake::Config::new(".")
        .no_build_target(true)
        .init_c_cfg(build.clone())
        .init_cxx_cfg(build)
        .build();

    println!("cargo:rerun-if-changed=src/*");
    println!("cargo:rerun-if-changed=include/*");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=hdkrsConfig.cmake");

    // Link against the cxxbridge
    println!("cargo:rustc-link-lib=static=hdkrs");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
