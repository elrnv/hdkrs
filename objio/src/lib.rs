use std::pin::Pin;

use gut::io::obj::*;
use gut::io::MeshExtractor;
use gut::mesh::topology::*;

#[cxx::bridge(namespace = "objio")]
mod ffi {
    #[namespace = ""]
    extern "C++" {
        include!("hdkrs/src/lib.rs.h");
        include!("rust/cxx.h");
        type GU_Detail = hdkrs::ffi::GU_Detail;
    }
    extern "Rust" {
        fn polymesh_to_obj_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn pointcloud_to_obj_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;

        fn add_obj_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]);
    }
}

use hdkrs::ffi::GU_Detail;

/// Extract a `PolyMesh` from the given detail and write it in Obj format into a
/// `ByteBuffer`.
pub fn polymesh_to_obj_buffer(detail: &GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_polymesh(detail).map(|mesh| {
        convert_polymesh_to_obj_format(&mesh.0)
            .map(write_obj)
            .unwrap_or_else(|_| Default::default())
    })
}
/// Extract a `PointCloud` from the given detail and write it in Obj format into a
/// `ByteBuffer`.
pub fn pointcloud_to_obj_buffer(detail: &GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_pointcloud(detail).map(|mesh| {
        convert_pointcloud_to_obj_format(&mesh.0)
            .map(write_obj)
            .unwrap_or_else(|_| Default::default())
    })
}

// Helper for writing obj data to a byte buffer.
fn write_obj(obj: ObjData) -> Vec<u8> {
    let mut vec_data = Vec::<u8>::new();
    obj.write_to_buf(&mut vec_data)
        .expect("Failed to write Obj data to byte buffer");
    vec_data
}

/// Parse a given byte array into a PolyMesh assuming obj format.
pub fn parse_obj_mesh(data: &[u8]) -> Box<hdkrs::Mesh> {
    if let Ok(obj_data) = ObjData::load_buf_with_config(data, LoadConfig { strict: false }) {
        if let Ok(mesh) = obj_data.extract_polymesh() {
            if mesh.num_faces() > 0 {
                return Box::new(mesh.into());
            }
        }
        if let Ok(mesh) = obj_data.extract_pointcloud() {
            return Box::new(mesh.into());
        }
    }
    Box::new(hdkrs::Mesh::None)
}

/// Parse a given byte array into a PolyMesh or a PointCloud and add it to the given detail.
pub fn add_obj_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]) {
    parse_obj_mesh(data).add_to_detail(detail);
}
