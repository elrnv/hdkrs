// Required for proper loading.
#include <UT/UT_DSOVersion.h>

#include <GU/GU_Detail.h>
#include <GEO/GEO_AttributeHandle.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IStream.h>
#include <SOP/SOP_Node.h>
#include <UT/UT_IOTable.h>
#include <boost/variant.hpp>
#include <hdkrs/io.h>
#include <hdkrs/mesh.h>
#include <hdkrs/hdkrs.h>
#include <iostream>

#include "add_mesh_visitor.h"
#include "GEO_ObjIO.h"

using namespace hdkrs;

GEO_IOTranslator *
GEO_ObjIO::duplicate() const
{
    return new GEO_ObjIO(*this);
}

const char *
GEO_ObjIO::formatName() const
{
    return "Wavefront OBJ format";
}

int
GEO_ObjIO::checkExtension(const char *name) 
{
    UT_String sname(name);
    if (sname.fileExtension() && !strcmp(sname.fileExtension(), ".obj"))
        return true;
    return false;
}

int
GEO_ObjIO::checkMagicNumber(unsigned magic)
{
    return 0;
}

GA_Detail::IOStatus
GEO_ObjIO::fileLoad(GEO_Detail *detail, UT_IStream &is, bool)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    UT_WorkBuffer buf;
    bool success = is.getAll(buf);
    if (!success)
        return GA_Detail::IOStatus(success);
    exint size = buf.length();
    auto data = buf.buffer();
    io::MeshVariant mesh = io::parse_obj_mesh(data, size);
    boost::apply_visitor( AddMesh(detail), std::move(mesh) );
    return GA_Detail::IOStatus(success);
}

GA_Detail::IOStatus
GEO_ObjIO::fileSave(const GEO_Detail *detail, std::ostream &os)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    OwnedPtr<HR_PolyMesh> polymesh = mesh::build_polymesh(static_cast<const GU_Detail*>(detail));
    if (polymesh) {
        auto buf = io::ByteBuffer::write_obj_mesh(std::move(polymesh));
        os.write(buf.data(), buf.size());
        return GA_Detail::IOStatus(true);
    }

    // If no polygons are found we try to save the pointcloud
    OwnedPtr<HR_PointCloud> pointcloud = mesh::build_pointcloud(static_cast<const GU_Detail*>(detail));
    if (pointcloud) {
        auto buf = io::ByteBuffer::write_obj_mesh(std::move(pointcloud));
        os.write(buf.data(), buf.size());
        return GA_Detail::IOStatus(true);
    }

    return GA_Detail::IOStatus(false);
}

void
newGeometryIO(void *)
{
    GU_Detail::registerIOTranslator(new GEO_ObjIO());
    UT_ExtensionList *geoextension;
    geoextension = UTgetGeoExtensions();
    if (!geoextension->findExtension("obj"))
        geoextension->addExtension("obj");
}
