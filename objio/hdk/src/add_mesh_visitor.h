#include <GU/GU_Detail.h>
#include <boost/variant.hpp>
#include <hdkrs/mesh.h>

struct AddMesh : public boost::static_visitor<bool>
{
    AddMesh(GEO_Detail* detail) : detail(static_cast<GU_Detail*>(detail)) {}
    bool operator()( hdkrs::OwnedPtr<HR_TetMesh> tetmesh ) const
    {
        hdkrs::mesh::add_tetmesh(detail, std::move(tetmesh));
        return true;
    }
    bool operator()( hdkrs::OwnedPtr<HR_PolyMesh> polymesh ) const
    {
        hdkrs::mesh::add_polymesh(detail, std::move(polymesh));
        return true;
    }
    bool operator()( boost::blank nothing ) const
    {
        return false;
    }

    GU_Detail* detail;
};

