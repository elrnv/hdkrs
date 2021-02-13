pub mod cffi;
pub mod interop;

#[cxx::bridge(namespace = "hdkrs")]
mod ffi {
    extern "Rust" {
        type PolyMesh;
    }
}
pub struct PolyMesh {
    pub mesh: gut::mesh::PolyMesh<f64>,
}
