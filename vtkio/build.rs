use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "vtkio";
    let build = cxx_build::bridge("src/lib.rs");
    cmake::Config::new(".")
        .no_build_target(true)
        .init_c_cfg(build.clone())
        .init_cxx_cfg(build)
        .build();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=src/lib.rs");
    let target_os = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    if target_os == "windows" {
        // For some reason linking on windows doesn't recognize the lib prefix automatically.
        println!("cargo:rustc-link-lib=static=libcxxbridge-vtkiohdk");
    } else {
        println!("cargo:rustc-link-lib=static=cxxbridge-vtkiohdk");
    }
    println!("cargo:rustc-link-search=native={}", out_dir);
}
