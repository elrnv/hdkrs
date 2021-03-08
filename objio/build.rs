use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "objio";
    cxx_build::bridge("src/lib.rs").compile("cxxbridge-objiohdk");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
