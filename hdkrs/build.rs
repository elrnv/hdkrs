extern crate cbindgen;
extern crate glob;

use glob::glob;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let output_file = out_dir
        .join(format!("{}.h", package_name))
        .display()
        .to_string();

    let mut config: cbindgen::Config = Default::default();

    config.include_guard = Some(String::from("HDKRS_CAPI_H"));
    config.line_length = 80;
    config.tab_width = 4;
    config.cpp_compat = true;
    config.language = cbindgen::Language::Cxx;

    cbindgen::generate_with_config(&crate_dir, config)
        .expect("Unable to generate bidnings for hdkrs")
        .write_to_file(&output_file);

    cxx_build::bridge("src/lib.rs")
        .file("src/hdkrs.C")
        .flag_if_supported("-std=c++14")
        .compile("hdkrs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/hdkrs.C");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=include/hdkrs.h");
    println!("cargo:rerun-if-changed=hdkrsConfig.cmake");

    let header_target = out_dir.join("hdkrs");
    if !header_target.exists() {
        fs::create_dir(&header_target).unwrap_or_else(|_| {
            panic!(
                "Failed to create target directory for header files: {:?}",
                header_target
            )
        });
    }

    // Copy HDK API C headers from source to target directory
    for entry in glob(&format!("{}/src/*.h", crate_dir)).expect("Failed to find headers.") {
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

    // Copy CMake config file so it can be used by other plugins

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
    println!("copying {:?} to {:?}", src, dst);
    fs::copy(&src, &dst)
        .unwrap_or_else(|_| panic!("Failed to copy cmake config {:?}", cmake_config_file));
}
