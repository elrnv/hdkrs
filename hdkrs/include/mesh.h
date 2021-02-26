#pragma once

#include <optional>

class GU_Detail;

namespace hdkrs {
namespace {

/**
 * Add the given meshes into the given detail
 */

void add_mesh(GU_Detail* detail, rust::Box<Mesh> mesh);
void add_polymesh(GU_Detail* detail, rust::Box<PolyMesh> polymesh);
void add_tetmesh(GU_Detail* detail, rust::Box<TetMesh> tetmesh);
void add_pointcloud(GU_Detail* detail, rust::Box<PointCloud> ptcloud);
void update_points(GU_Detail* detail, rust::Box<PointCloud> ptcloud);

std::optional<rust::Box<TetMesh>> build_tetmesh(const GU_Detail *detail);
std::optional<rust::Box<PolyMesh>> build_polymesh(const GU_Detail* detail);
std::optional<rust::Box<PointCloud>> build_pointcloud(const GU_Detail* detail);

}
} // namespace hdkrs

#include "mesh-inl.h"