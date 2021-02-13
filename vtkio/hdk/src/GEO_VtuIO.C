#include <GU/GU_Detail.h>
#include <GEO/GEO_AttributeHandle.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IStream.h>
#include <SOP/SOP_Node.h>
#include <UT/UT_IOTable.h>
#include <boost/variant.hpp>
#include <hdkrs/io.h>
#include <hdkrs/mesh.h>
#include <iostream>

#include "add_mesh_visitor.h"
#include "GEO_VtuIO.h"

using namespace hdkrs;

GEO_IOTranslator *
GEO_VtuIO::duplicate() const
{
    return new GEO_VtuIO(*this);
}

const char *
GEO_VtuIO::formatName() const
{
    return "Visualization ToolKit (VTK) Unstructured Grid in XML Format";
}

int
GEO_VtuIO::checkExtension(const char *name) 
{
    UT_String sname(name);
    if (sname.fileExtension() &&
        (!strcmp(sname.fileExtension(), ".vtu") ||
         !strcmp(sname.fileExtension(), ".pvtu")))
        return true;
    return false;
}

int
GEO_VtuIO::checkMagicNumber(unsigned magic)
{
    return 0;
}

GA_Detail::IOStatus
GEO_VtuIO::fileLoad(GEO_Detail *detail, UT_IStream &is, bool)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    UT_WorkBuffer buf;
    bool success = is.getAll(buf);
    if (!success)
        return GA_Detail::IOStatus(success);
    exint size = buf.length();
    auto data = buf.buffer();
    io::MeshVariant mesh = io::parse_vtu_mesh(data, size);
    boost::apply_visitor( AddMesh(detail), std::move(mesh) );
    return GA_Detail::IOStatus(success);
}

GA_Detail::IOStatus
GEO_VtuIO::fileSave(const GEO_Detail *detail, std::ostream &os)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    // Try to save the tetmesh first
    OwnedPtr<HR_TetMesh> tetmesh = mesh::build_tetmesh(static_cast<const GU_Detail*>(detail));
    if (tetmesh) {
        auto buf = io::ByteBuffer::write_vtu_mesh(std::move(tetmesh));
        os.write(buf.data(), buf.size());
        return GA_Detail::IOStatus(true);
    }

    // If no tets are found we try to save the polymesh
    OwnedPtr<HR_PolyMesh> polymesh = mesh::build_polymesh(static_cast<const GU_Detail*>(detail));
    if (polymesh) {
        auto buf = io::ByteBuffer::write_vtu_mesh(std::move(polymesh));
        os.write(buf.data(), buf.size());
        return GA_Detail::IOStatus(true);
    }

    // If no polygons are found we try to save the pointcloud
    OwnedPtr<HR_PointCloud> pointcloud = mesh::build_pointcloud(static_cast<const GU_Detail*>(detail));
    if (pointcloud) {
        auto buf = io::ByteBuffer::write_vtu_mesh(std::move(pointcloud));
        os.write(buf.data(), buf.size());
        return GA_Detail::IOStatus(true);
    }

    return GA_Detail::IOStatus(false);
}
