#pragma once

#include "rust/cxx.h"
#include <memory>

class GU_Detail;

// Ensure that Detail can be used through a shared ptr over the Rust ffi boundary.
void impl_shared_ptr(std::shared_ptr<GU_Detail> detail);

namespace hdkrs {
    struct Mesh;
    struct UnstructuredMesh;
    struct TetMesh;
    struct PolyMesh;
    struct PointCloud;

    /**
    * Add the given meshes into the given detail
    */

    void add_unstructured_mesh(GU_Detail& detail, const UnstructuredMesh& mesh);
    void add_polymesh(GU_Detail& detail, const PolyMesh& polymesh);
    void add_tetmesh(GU_Detail& detail, const TetMesh& tetmesh);
    void add_pointcloud(GU_Detail& detail, const PointCloud& ptcloud);
    void update_points(GU_Detail& detail, const PointCloud& ptcloud);

    rust::box<UnstructuredMesh> build_unstructured_mesh(const GU_Detail& detail);
    rust::box<TetMesh> build_tetmesh(const GU_Detail& detail);
    rust::box<PolyMesh> build_polymesh(const GU_Detail& detail);
    rust::box<PointCloud> build_pointcloud(const GU_Detail& detail);

} // namespace hdkrs