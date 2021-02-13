// Required for proper loading.
#include <UT/UT_DSOVersion.h>

#include <GU/GU_Detail.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IOTable.h>

#include <vector>

#include "GEO_VtkIO.h"
#include "GEO_VtuIO.h"
#include "GEO_VtpIO.h"

void
newGeometryIO(void *)
{
    GU_Detail::registerIOTranslator(new GEO_VtkIO());
    GU_Detail::registerIOTranslator(new GEO_VtuIO());
    GU_Detail::registerIOTranslator(new GEO_VtpIO());
    std::vector<const char *> extensions{"vtk", "vtu", "vtp", "pvtp", "pvtu"};

    UT_ExtensionList *geoextension;
    geoextension = UTgetGeoExtensions();

    for (auto ext : extensions) {
        if (!geoextension->findExtension(ext))
            geoextension->addExtension(ext);
    }
}
