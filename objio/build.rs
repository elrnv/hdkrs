use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "objio";
    let build = cxx_build::bridge("src/lib.rs");
    //.file("../target/cxxbridge/hdkrs/src/lib.rs.cc")
    //    .compile("cxxbridge-objiohdk");
    cmake::Config::new(".")
        .no_build_target(true)
        .init_c_cfg(build.clone())
        .init_cxx_cfg(build)
        .build();

    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rustc-link-lib=static=cxxbridge-objiohdk");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
