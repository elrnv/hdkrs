#pragma once

#include <boost/variant.hpp>

namespace hdkrs {
namespace io {
namespace {

/*
 * VTK
 */

inline ByteBuffer ByteBuffer::write_vtp_mesh(OwnedPtr<HR_PolyMesh> polymesh) {
    return hr_make_polymesh_vtp_buffer(polymesh.get());
}

inline ByteBuffer ByteBuffer::write_vtp_mesh(OwnedPtr<HR_PointCloud> ptcloud) {
    return hr_make_pointcloud_vtp_buffer(ptcloud.get());
}

inline ByteBuffer ByteBuffer::write_vtu_mesh(OwnedPtr<HR_PolyMesh> polymesh) {
    return hr_make_polymesh_vtu_buffer(polymesh.get());
}

inline ByteBuffer ByteBuffer::write_vtu_mesh(OwnedPtr<HR_TetMesh> tetmesh) {
    return hr_make_tetmesh_vtu_buffer(tetmesh.get());
}

inline ByteBuffer ByteBuffer::write_vtu_mesh(OwnedPtr<HR_PointCloud> ptcloud) {
    return hr_make_pointcloud_vtu_buffer(ptcloud.get());
}

inline ByteBuffer ByteBuffer::write_vtk_mesh(OwnedPtr<HR_PolyMesh> polymesh) {
    return hr_make_polymesh_vtk_buffer(polymesh.get());
}

inline ByteBuffer ByteBuffer::write_vtk_mesh(OwnedPtr<HR_TetMesh> tetmesh) {
    return hr_make_tetmesh_vtk_buffer(tetmesh.get());
}

inline ByteBuffer ByteBuffer::write_vtk_mesh(OwnedPtr<HR_PointCloud> ptcloud) {
    return hr_make_pointcloud_vtk_buffer(ptcloud.get());
}

[[maybe_unused]]
MeshVariant parse_vtp_mesh(const char * data, std::size_t size) {
    MeshVariant ret((boost::blank()));
    HR_Mesh mesh = hr_parse_vtp_mesh(data, size);
    switch (mesh.tag) {
        case HRMeshType::HR_POLYMESH:
            ret = OwnedPtr<HR_PolyMesh>(mesh.polymesh);
            break;
        default: break;
    }
    return ret;
}

[[maybe_unused]]
MeshVariant parse_vtu_mesh(const char * data, std::size_t size) {
    MeshVariant ret((boost::blank()));
    HR_Mesh mesh = hr_parse_vtu_mesh(data, size);
    switch (mesh.tag) {
        case HRMeshType::HR_TETMESH:
            ret = OwnedPtr<HR_TetMesh>(mesh.tetmesh);
            break;
        case HRMeshType::HR_POLYMESH:
            ret = OwnedPtr<HR_PolyMesh>(mesh.polymesh);
            break;
        default: break;
    }
    return ret;
}

[[maybe_unused]]
MeshVariant parse_vtk_mesh(const char * data, std::size_t size) {
    MeshVariant ret((boost::blank()));
    HR_Mesh mesh = hr_parse_vtk_mesh(data, size);
    switch (mesh.tag) {
        case HRMeshType::HR_TETMESH:
            ret = OwnedPtr<HR_TetMesh>(mesh.tetmesh);
            break;
        case HRMeshType::HR_POLYMESH:
            ret = OwnedPtr<HR_PolyMesh>(mesh.polymesh);
            break;
        default: break;
    }
    return ret;
}

/*
 * Obj
 */

inline ByteBuffer ByteBuffer::write_obj_mesh(OwnedPtr<HR_PolyMesh> polymesh) {
    return hr_make_polymesh_obj_buffer(polymesh.get());
}

inline ByteBuffer ByteBuffer::write_obj_mesh(OwnedPtr<HR_PointCloud> ptcloud) {
    return hr_make_pointcloud_obj_buffer(ptcloud.get());
}

[[maybe_unused]]
MeshVariant parse_obj_mesh(const char * data, std::size_t size) {
    MeshVariant ret((boost::blank()));
    HR_Mesh mesh = hr_parse_obj_mesh(data, size);
    switch (mesh.tag) {
        case HRMeshType::HR_POLYMESH:
            ret = OwnedPtr<HR_PolyMesh>(mesh.polymesh);
            break;
        default: break;
    }
    return ret;
}

} // namespace (static)
} // namespace io
} // namespace hdkrs
