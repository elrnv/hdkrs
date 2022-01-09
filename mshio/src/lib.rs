use std::pin::Pin;

use meshx::io::msh::*;
use meshx::io::MeshExtractor;

#[cxx::bridge(namespace = "mshio")]
mod ffi {
    #[namespace = ""]
    extern "C++" {
        include!("hdkrs/src/lib.rs.h");
        include!("rust/cxx.h");
        type GU_Detail = hdkrs::ffi::GU_Detail;
    }
    extern "Rust" {
        // Export currently unsupported.

        fn add_msh_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]);
    }
}

use hdkrs::ffi::GU_Detail;

/// Parse a given byte array into a Mesh assuming msh format.
pub fn parse_msh_mesh(data: &[u8]) -> Box<hdkrs::Mesh> {
    if let Ok(msh) = parse_msh_bytes(data) {
        if let Ok(mesh) = <MshFile<u64, i32, f64> as MeshExtractor<f64>>::extract_mesh(&msh) {
            return Box::new(hdkrs::Mesh::from(mesh));
        }
    }
    Box::new(hdkrs::Mesh::None)
}

/// Parse a given byte array into a Mesh and add it to the given detail.
pub fn add_msh_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]) {
    parse_msh_mesh(data).add_to_detail(detail);
}
