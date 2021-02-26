use glob::glob;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    //let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build the cxx bridge

    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .compile("cxxbridge-hdkrs");

    // Copy HDK API C headers from source to target directory

    let header_target = out_dir.join("hdkrs");
    if !header_target.exists() {
        fs::create_dir(&header_target).unwrap_or_else(|_| {
            panic!(
                "Failed to create target directory for header files: {:?}",
                header_target
            )
        });
    }

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

    println!("cargo:rustc-link-lib=static=cxxbridge-hdkrs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=hdkrsConfig.cmake");
}
