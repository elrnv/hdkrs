use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "vtkio";

    cxx_build::bridge("src/lib.rs").compile("cxxbridge-vtkiohdk");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
