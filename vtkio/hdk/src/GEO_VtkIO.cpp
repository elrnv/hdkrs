#include <iostream>

#include <GU/GU_Detail.h>
#include <GEO/GEO_AttributeHandle.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IStream.h>
#include <SOP/SOP_Node.h>
#include <UT/UT_IOTable.h>

#include <vtkio/src/lib.rs.h>
#include <hdkrs/prelude.h>

#include "GEO_VtkIO.h"

GEO_IOTranslator *
GEO_VtkIO::duplicate() const
{
    return new GEO_VtkIO(*this);
}

const char *
GEO_VtkIO::formatName() const
{
    return "Visualization ToolKit (VTK) Legacy Format";
}

int
GEO_VtkIO::checkExtension(const char *name) 
{
    UT_String sname(name);
    if (sname.fileExtension() && !strcmp(sname.fileExtension(), ".vtk"))
        return true;
    return false;
}

int
GEO_VtkIO::checkMagicNumber(unsigned magic)
{
    return 0;
}

GA_Detail::IOStatus
GEO_VtkIO::fileLoad(GEO_Detail *detail, UT_IStream &is, bool)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    UT_WorkBuffer buf;
    bool success = is.getAll(buf);
    if (!success)
        return GA_Detail::IOStatus(success);
    rust::Slice<const uint8_t> slice(reinterpret_cast<const unsigned char*>(buf.buffer()), buf.length());
    vtkio::add_vtk_mesh(*static_cast<GU_Detail*>(detail), slice);
    return GA_Detail::IOStatus(success);
}

GA_Detail::IOStatus
GEO_VtkIO::fileSave(const GEO_Detail *detail, std::ostream &os)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    const GU_Detail &gud = *static_cast<const GU_Detail*>(detail);

    // Try to save unstructured mesh first
    try {
        auto buf = vtkio::mesh_to_vtk_buffer(gud);
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(...) {}

    // If no polygons are found we try to save the pointcloud
    try {
        auto buf = vtkio::pointcloud_to_vtk_buffer(gud);
        os.write(reinterpret_cast<const char *>(buf.data()), buf.size());
        return GA_Detail::IOStatus(true);
    } catch(...) { }

    return GA_Detail::IOStatus(false);
}
