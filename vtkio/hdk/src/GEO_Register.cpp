#include <vector>


#include <GU/GU_Detail.h>
#include <GEO/GEO_IOTranslator.h>
#include <UT/UT_IOTable.h>

#include "GEO_VtkIO.h"
#include "GEO_VtuIO.h"
#include "GEO_VtpIO.h"


#include <SYS/SYS_Types.h>
#include <SYS/SYS_Version.h>
#include <SYS/SYS_Visibility.h>

#define UT_DSO_VERSION  SYS_VERSION_RELEASE

extern "C" {
    class UT_DSOInfo
    {
        public:
        /*
         * Linux only:
         * Set to true if this DSO should be loaded with RTLD_GLOBAL.
         * Note that this will mean that your DSO is dlclose()'ed, and then
         * re-dlopen()'ed.
         *
         * If set to true, then HoudiniDSOInit will be called again after the
         * DSO is re-openend.  The UT_DSOInfo struct will have its loadGlobal
         * member set to true.
         */
        bool loadGlobal;
    };

    SYS_VISIBILITY_EXPORT void __HoudiniDSOVersion(const char **version_string);
    SYS_VISIBILITY_EXPORT void __HoudiniGetTagInfo(const char **version_string);

    /*
     * If your DSO implements this function, it will be called immediately
     * after the DSO is loaded.
     *
     * dsoinfo.loadGlobal will initially be set to false, but see comment above
     *
     */
    SYS_VISIBILITY_EXPORT void __HoudiniDSOInit(UT_DSOInfo& dsoinfo);
}

#ifdef MAKING_DSO

extern "C" SYS_VISIBILITY_EXPORT void
__HoudiniDSOVersion(const char **v) { *v = UT_DSO_VERSION; }
#ifdef UT_DSO_TAGINFO
extern "C" SYS_VISIBILITY_EXPORT void
__HoudiniGetTagInfo(const char **v) { *v = UT_DSO_TAGINFO; }
#endif

#endif // MAKING_DSO

#ifdef MAKING_DSO
#define UT_DSOVERSION_EXPORT extern "C" SYS_VISIBILITY_EXPORT
#else
#define UT_DSOVERSION_EXPORT static inline
#endif

#if defined(__GNUC__)
UT_DSOVERSION_EXPORT unsigned
__HoudiniCompilerVersion() { return (__GNUC__ * 100 + __GNUC_MINOR__); }
#elif defined(_MSC_VER)
UT_DSOVERSION_EXPORT unsigned
__HoudiniCompilerVersion() { return (_MSC_VER); }
#else
#error Unsupported compiler
#endif


extern "C" void
__newGeometryIO(void *)
{
    GU_Detail::registerIOTranslator(new GEO_VtkIO());
    GU_Detail::registerIOTranslator(new GEO_VtuIO());
    GU_Detail::registerIOTranslator(new GEO_VtpIO());
    std::vector<const char *> extensions{"vtk", "vtu", "vtp", "pvtp", "pvtu"};

    std::cerr << "REGISTERED" << std::endl;

    UT_ExtensionList *geoextension;
    geoextension = UTgetGeoExtensions();

    for (auto ext : extensions) {
        if (!geoextension->findExtension(ext))
            geoextension->addExtension(ext);
    }
}
