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

// Rust interface into mshio.
#include <mshio/src/lib.rs.h>

#include "GEO_MshIO.h"

using namespace std;

GEO_IOTranslator *
GEO_MshIO::duplicate() const
{
    return new GEO_MshIO(*this);
}

const char *
GEO_MshIO::formatName() const
{
    return "MSH format";
}

int
GEO_MshIO::checkExtension(const char *name) 
{
    UT_String sname(name);
    if (sname.fileExtension() && !strcmp(sname.fileExtension(), ".msh"))
        return true;
    return false;
}

int
GEO_MshIO::checkMagicNumber(unsigned magic)
{
    return 0;
}

GA_Detail::IOStatus
GEO_MshIO::fileLoad(GEO_Detail *detail, UT_IStream &is, bool)
{
    if (!detail) // nothing to do
        return GA_Detail::IOStatus(true);

    UT_WorkBuffer buf;
    bool success = is.getAll(buf);
    if (!success)
        return GA_Detail::IOStatus(success);
    rust::Slice<const uint8_t> slice(reinterpret_cast<const unsigned char*>(buf.buffer()), buf.length());
    mshio::add_msh_mesh(static_cast<GU_Detail &>(*detail), slice);
    return GA_Detail::IOStatus(success);
}

GA_Detail::IOStatus
GEO_MshIO::fileSave(const GEO_Detail *detail, std::ostream &os)
{
    // Export unsupported.
    return GA_Detail::IOStatus(false);
}

void
newGeometryIO(void *)
{
    GU_Detail::registerIOTranslator(new GEO_MshIO());
    UT_ExtensionList *geoextension;
    geoextension = UTgetGeoExtensions();
    if (!geoextension->findExtension("msh"))
        geoextension->addExtension("msh");
}
