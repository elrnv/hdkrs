use std::pin::Pin;

use meshx::io::vtk::*;
use meshx::io::MeshExtractor;
use meshx::mesh::topology::*;
use model::Vtk;

#[cxx::bridge(namespace = "vtkio")]
mod ffi {
    #[namespace = ""]
    extern "C++" {
        include!("hdkrs/src/lib.rs.h");
        type GU_Detail = hdkrs::ffi::GU_Detail;
    }
    extern "Rust" {
        fn polymesh_to_vtk_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn polymesh_to_vtu_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn polymesh_to_vtp_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;

        fn tetmesh_to_vtu_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn tetmesh_to_vtk_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;

        fn pointcloud_to_vtu_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn pointcloud_to_vtp_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn pointcloud_to_vtk_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;

        fn mesh_to_vtu_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;
        fn mesh_to_vtk_buffer(detail: &GU_Detail) -> Result<Vec<u8>>;

        fn add_vtp_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]);
        fn add_vtu_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]);
        fn add_vtk_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]);
    }
}

use hdkrs::ffi::GU_Detail;

#[derive(Debug)]
struct Error(String);
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", &self.0)
    }
}

// Helper for writing legacy vtk to a byte buffer.
fn write_legacy_vtk(vtk: Vtk) -> Vec<u8> {
    let mut vec_data = Vec::<u8>::new();
    vtk.write_legacy(&mut vec_data)
        .expect("Failed to write Vtk data to byte buffer");
    vec_data
}
// Helper for writing xml vtk to a byte buffer.
fn write_xml_vtk(vtk: Vtk) -> Vec<u8> {
    let mut vec_data = Vec::<u8>::new();
    vtk.write_xml(&mut vec_data)
        .expect("Failed to write Vtk data to byte buffer");
    vec_data
}

/// Extract a PolyMesh from the given detail and write it as a polygon mesh in XML VTK format
/// returned through an appropriately sized `ByteBuffer`.
pub fn polymesh_to_vtp_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_polymesh(detail).map(|mesh| {
        convert_polymesh_to_vtk_format(&mesh.0, VTKPolyExportStyle::PolyData)
            .map(write_xml_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}
/// Extract a PolyMesh from the given detail and write it as an unstructured grid in XML VTK format
/// returned through an appropriately sized `ByteBuffer`.
pub fn polymesh_to_vtu_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_polymesh(detail).map(|mesh| {
        convert_polymesh_to_vtk_format(&mesh.0, VTKPolyExportStyle::UnstructuredGrid)
            .map(write_xml_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}
/// Extract a PolyMesh from the given detail and write it into a binary VTK format returned through
/// an appropriately sized `ByteBuffer`.
pub fn polymesh_to_vtk_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_polymesh(detail).map(|mesh| {
        convert_polymesh_to_vtk_format(&mesh.0, VTKPolyExportStyle::PolyData)
            .map(write_legacy_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}

/// Extract a PointCloud from the given detail and write it as a polygon mesh in
/// XML VTK format returned through an appropriately sized `ByteBuffer`.
pub fn pointcloud_to_vtp_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_pointcloud(detail).map(|mesh| {
        convert_pointcloud_to_vtk_format(&mesh.0, VTKPolyExportStyle::PolyData)
            .map(write_xml_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}
/// Extract a PointCloud from the given detail and write it as an unstructured
/// grid in XML VTK format returned through an appropriately sized `ByteBuffer`.
pub fn pointcloud_to_vtu_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_pointcloud(detail).map(|mesh| {
        convert_pointcloud_to_vtk_format(&mesh.0, VTKPolyExportStyle::UnstructuredGrid)
            .map(write_xml_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}
/// Extract a PointCloud from the given detail and write it into a binary VTK
/// format returned through an appropriately sized `ByteBuffer`.
pub fn pointcloud_to_vtk_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_pointcloud(detail).map(|mesh| {
        convert_pointcloud_to_vtk_format(&mesh.0, VTKPolyExportStyle::PolyData)
            .map(write_legacy_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}

/// Extract a TetMesh from the given detail and write it as an unstructured grid
/// in XML VTK format returned through an appropriately sized `ByteBuffer`.
pub fn tetmesh_to_vtu_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_tetmesh(detail).map(|mesh| {
        convert_tetmesh_to_vtk_format(&mesh.0)
            .map(write_xml_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}

/// Extract a TetMesh from the given detail and write it into a binary VTK
/// format returned through an appropriately sized `ByteBuffer`.
pub fn tetmesh_to_vtk_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_tetmesh(detail).map(|mesh| {
        convert_tetmesh_to_vtk_format(&mesh.0)
            .map(write_legacy_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}

/// Extract an unstructured mesh from the given detail and write it as an unstructured grid
/// in XML VTK format returned through an appropriately sized `ByteBuffer`.
pub fn mesh_to_vtu_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_unstructured_mesh(detail).map(|mesh| {
        convert_mesh_to_vtk_format(&mesh.0)
            .map(write_xml_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}

/// Extract an unstructured mesh from the given detail and write it into a binary VTK
/// format returned through an appropriately sized `ByteBuffer`.
pub fn mesh_to_vtk_buffer(detail: &hdkrs::ffi::GU_Detail) -> Result<Vec<u8>, cxx::Exception> {
    hdkrs::ffi::build_unstructured_mesh(detail).map(|mesh| {
        convert_mesh_to_vtk_format(&mesh.0)
            .map(write_legacy_vtk)
            .unwrap_or_else(|_| Default::default())
    })
}

/// Helper to convert the given VTK data set into a valid `Mesh` type representing an unstructured
/// mesh.
///
/// In case of failure `None` is returned.
fn convert_vtk_to_mesh(vtk: &Vtk) -> hdkrs::Mesh {
    if let Ok(mesh) = vtk.extract_mesh() {
        if mesh.num_cells() > 0 {
            return mesh.into();
        }
    }
    hdkrs::Mesh::None
}

/// Helper to convert the given VTK data set into a valid `Mesh` type representing a PolyMesh.
///
/// In case of failure `None` is returned.
fn convert_vtk_to_polymesh(vtk: &Vtk) -> hdkrs::Mesh {
    if let Ok(mesh) = vtk.extract_polymesh() {
        if mesh.num_faces() > 0 {
            return mesh.into();
        }
    }
    hdkrs::Mesh::None
}

/// Helper to convert the given VTK data set into a valid `Mesh` type representing a PointCloud.
///
/// In case of failure `None` is returned.
fn convert_vtk_to_pointcloud(vtk: &Vtk) -> hdkrs::Mesh {
    vtk.extract_pointcloud().ok().into()
}

/// Parse a given byte array into a Mesh and add it to the given detail.
pub fn add_vtp_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]) {
    parse_vtp_mesh(data).add_to_detail(detail);
}

/// Parse a given byte array into a TetMesh or a PolyMesh and add it to the given detail.
pub fn add_vtu_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]) {
    parse_vtu_mesh(data).add_to_detail(detail);
}

/// Parse a given byte array into a TetMesh or a PolyMesh and add it to the given detail.
pub fn add_vtk_mesh(detail: Pin<&mut GU_Detail>, data: &[u8]) {
    parse_vtk_mesh(data).add_to_detail(detail);
}

/// Parse a given byte array into a PolyMesh depending on what is stored in the
/// buffer assuming polygon VTK format.
pub fn parse_vtp_mesh(data: &[u8]) -> Box<hdkrs::Mesh> {
    if let Ok(vtk) = Vtk::parse_xml(data) {
        if let Ok(mesh) = vtk.extract_polymesh() {
            if mesh.num_faces() > 0 {
                return Box::new(mesh.into());
            }
        }
        if let Ok(mesh) = vtk.extract_pointcloud() {
            return Box::new(mesh.into());
        }
    }
    Box::new(hdkrs::Mesh::None)
}

/// Parse a given byte array into a TetMesh or a PolyMesh depending on what is stored in the
/// buffer assuming unstructured grid VTK format.
pub fn parse_vtu_mesh(data: &[u8]) -> Box<hdkrs::Mesh> {
    if let Ok(vtk) = Vtk::parse_xml(data) {
        return Box::new(
            convert_vtk_to_mesh(&vtk)
                .or_else(|| convert_vtk_to_polymesh(&vtk))
                .or_else(|| convert_vtk_to_pointcloud(&vtk))
                .or(hdkrs::Mesh::None),
        );
    }
    Box::new(hdkrs::Mesh::None)
}

/// Parse a given byte array into a TetMesh or a PolyMesh depending on what is stored in the
/// buffer assuming VTK format.
pub fn parse_vtk_mesh(data: &[u8]) -> Box<hdkrs::Mesh> {
    if let Ok(vtk) = Vtk::parse_legacy_be(data) {
        return Box::new(
            convert_vtk_to_mesh(&vtk)
                .or_else(|| convert_vtk_to_polymesh(&vtk))
                .or_else(|| convert_vtk_to_pointcloud(&vtk))
                .or(hdkrs::Mesh::None),
        );
    }
    Box::new(hdkrs::Mesh::None)
}
