extern crate cbindgen;
extern crate glob;

use glob::glob;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let target_dir = target_dir(&package_name);

    let output_file = target_dir
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
        .compile("cxxbridge-hdkrs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/hdkrs.C");
    println!("cargo:rerun-if-changed=include/hdkrs.h");

    // Copy HDK API C headers from source to target directory
    for entry in glob(&format!("{}/src/*.h", crate_dir)).expect("Failed to find headers.") {
        match entry {
            Ok(src) => {
                let header = src.file_name().unwrap();
                let dst = target_dir.join(Path::new(&header));
                println!("copying {:?} to {:?}", src, dst);
                fs::copy(&src, &dst)
                    .unwrap_or_else(|_| panic!("Failed to copy header {:?}", header));
            }
            Err(e) => println!("{:?}", e),
        }
    }

    // Copy CMake config file so it can be used by other plugins

    let cmake_target = target_dir.parent().unwrap().join("cmake");
    if !cmake_target.is_dir() {
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

fn target_dir(package_name: &str) -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut target_dir = out_dir.as_path();
    for _ in 0..3 {
        assert!(target_dir.is_dir());
        target_dir = target_dir.parent().unwrap();
    }
    let target_dir = target_dir.join(package_name.to_string());
    if !target_dir.is_dir() {
        fs::create_dir(&target_dir)
            .unwrap_or_else(|_| panic!("Failed to create target directory: {:?}", target_dir));
    }

    target_dir
}
