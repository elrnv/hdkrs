set( objio_LIB_DIR "${CARGO_TARGET_DIR}" )

find_path( objiors_INCLUDE_DIR objio/src/lib.rs.h PATHS "${CARGO_TARGET_DIR}/../cxxbridge" DOC "objiors include directory")
find_library( objiors_LIBRARY objiors PATHS ${objio_LIB_DIR} DOC "objiors library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args( objiors REQUIRED_VARS objiors_LIBRARY objiors_INCLUDE_DIR )

if( objiors_FOUND )
    set( objiors_INCLUDE_DIRS ${objiors_INCLUDE_DIR} )
    set( objiors_LIBRARIES ${objiors_LIBRARY} )
endif( objiors_FOUND )
