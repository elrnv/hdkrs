#include <iostream>

#include <GU/GU_Detail.h>
#include <GEO/GEO_AttributeHandle.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IStream.h>
#include <SOP/SOP_Node.h>
#include <UT/UT_IOTable.h>

#include <vtkio/src/lib.rs.h>
#include <hdkrs/prelude.h>

#include "GEO_VtuIO.h"

using hdkrs::cast_box;

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
    rust::Slice<const uint8_t> slice(reinterpret_cast<const unsigned char*>(buf.buffer()), buf.length());
    vtkio::add_vtu_mesh(static_cast<GU_Detail&>(*detail), slice);
    return GA_Detail::IOStatus(success);
}

GA_Detail::IOStatus
GEO_VtuIO::fileSave(const GEO_Detail *detail, std::ostream &os)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    // Try to save the tetmesh first
    try {
        auto buf = vtkio::tetmesh_to_vtu_buffer(static_cast<const GU_Detail&>(*detail));
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(const std::runtime_error& e) {}

    // If no tets are found we try to save the polymesh
    try {
        auto buf = vtkio::polymesh_to_vtu_buffer(static_cast<const GU_Detail&>(*detail));
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(const std::runtime_error& e) {}

    // If no polygons are found we try to save the pointcloud
    try {
        auto buf = vtkio::pointcloud_to_vtu_buffer(static_cast<const GU_Detail&>(*detail));
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(const std::runtime_error& e) {}

    return GA_Detail::IOStatus(false);
}
