#include <iostream>
#include <optional>

// Required for proper loading.
#include <UT/UT_DSOVersion.h>
#include <GU/GU_Detail.h>
#include <GEO/GEO_AttributeHandle.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IStream.h>
#include <SOP/SOP_Node.h>
#include <UT/UT_IOTable.h>

// All relevant hdkrs headers. This includes HDK specific headers and the cxx bridge headers.
#include <hdkrs/prelude.h>

// Rust interface into objio.
#include <objio/src/lib.rs.h>

#include "GEO_ObjIO.h"

using namespace std;

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
    rust::Slice<const uint8_t> slice(reinterpret_cast<const unsigned char*>(buf.buffer()), buf.length());
    objio::add_obj_mesh(static_cast<GU_Detail &>(*detail), slice);
    return GA_Detail::IOStatus(success);
}

GA_Detail::IOStatus
GEO_ObjIO::fileSave(const GEO_Detail *detail, std::ostream &os)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    try {
        auto buf = objio::polymesh_to_obj_buffer(static_cast<const GU_Detail&>(*detail));
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(const std::runtime_error& e) {}

    // If no polygons are found we try to save the pointcloud
    try {
        auto buf = objio::pointcloud_to_obj_buffer(static_cast<const GU_Detail&>(*detail));
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(const std::runtime_error& e) {}

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
