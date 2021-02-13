#pragma once

#include <hdkrs/mesh.h>
#include <hdkrs/hdkrs.h>
#include <boost/variant.hpp>

namespace hdkrs {
namespace io {
namespace {

class ByteBuffer {
public:
    ByteBuffer(HR_ByteBuffer buf) : _data(buf.data), _size(buf.size) {}
    ByteBuffer(ByteBuffer&& buf) : _data(buf._data), _size(buf._size) {
        buf._data = nullptr;
        buf._size = 0;
    }
    ~ByteBuffer() {
        HR_ByteBuffer buf;
        buf.data = _data;
        buf.size = _size;
        hr_free_byte_buffer(buf);
    }

    ByteBuffer& operator=(ByteBuffer && other) {
        if (this != &other) {
            this->~ByteBuffer();

            _data = other._data;
            _size = other._size;
            other._data = nullptr;
            other._size = 0;
        }
        return *this;
    }

    // Get pointer to the allocated buffer data.
    const char * data() { return _data; }

    // Get size of the allocated buffer (number of bytes).
    std::size_t size() { return _size; }

    /**
     * Read the given meshes into an owned buffer.
     */
    static ByteBuffer write_vtp_mesh(OwnedPtr<HR_PolyMesh> polymesh);
    static ByteBuffer write_vtp_mesh(OwnedPtr<HR_PointCloud> ptcloud);
    static ByteBuffer write_vtu_mesh(OwnedPtr<HR_PolyMesh> polymesh);
    static ByteBuffer write_vtu_mesh(OwnedPtr<HR_TetMesh> tetmesh);
    static ByteBuffer write_vtu_mesh(OwnedPtr<HR_PointCloud> ptcloud);
    static ByteBuffer write_vtk_mesh(OwnedPtr<HR_PolyMesh> polymesh);
    static ByteBuffer write_vtk_mesh(OwnedPtr<HR_TetMesh> tetmesh);
    static ByteBuffer write_vtk_mesh(OwnedPtr<HR_PointCloud> ptcloud);
    static ByteBuffer write_obj_mesh(OwnedPtr<HR_PolyMesh> polymesh);
    static ByteBuffer write_obj_mesh(OwnedPtr<HR_PointCloud> ptcloud);

private:
    ByteBuffer(const ByteBuffer&) = delete; // Byte buffer is move only

    const char * _data;
    std::size_t  _size;
};

using MeshVariant = boost::variant<boost::blank, OwnedPtr<HR_PolyMesh>, OwnedPtr<HR_TetMesh>>;

MeshVariant parse_vtp_mesh(const char * data, std::size_t size);
MeshVariant parse_vtu_mesh(const char * data, std::size_t size);
MeshVariant parse_vtk_mesh(const char * data, std::size_t size);
MeshVariant parse_obj_mesh(const char * data, std::size_t size);

} // namespace (static)
} // namespace io
} // namespace hdkrs

#include "io-inl.h"
