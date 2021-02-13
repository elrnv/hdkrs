#pragma once

#include <hdkrs/hdkrs.h>
#include <hdkrs/pointer.h>

class GU_Detail;

namespace hdkrs {
namespace mesh {
namespace {

/**
 * Add the given meshes into the given detail
 */

void add_polymesh(GU_Detail* detail, OwnedPtr<HR_PolyMesh> polymesh);
void add_tetmesh(GU_Detail* detail, OwnedPtr<HR_TetMesh> tetmesh);
void add_pointcloud(GU_Detail* detail, OwnedPtr<HR_PointCloud> ptcloud);
void update_points(GU_Detail* detail, OwnedPtr<HR_PointCloud> ptcloud);

OwnedPtr<HR_TetMesh> build_tetmesh(const GU_Detail *detail);

OwnedPtr<HR_PolyMesh> build_polymesh(const GU_Detail* detail);

OwnedPtr<HR_PointCloud> build_pointcloud(const GU_Detail* detail);

} // namespace (static)
} // namespace mesh
} // namespace hdkrs

#include "mesh-inl.h"
