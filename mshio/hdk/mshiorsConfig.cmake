set( mshio_LIB_DIR "${CARGO_TARGET_DIR}" )

find_path( mshiors_INCLUDE_DIR mshio/src/lib.rs.h PATHS "${CARGO_TARGET_DIR}/../cxxbridge" DOC "mshiors include directory")
find_library( mshiors_LIBRARY mshiors PATHS ${mshio_LIB_DIR} DOC "mshiors library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args( mshiors REQUIRED_VARS mshiors_LIBRARY mshiors_INCLUDE_DIR )

if( mshiors_FOUND )
    set( mshiors_INCLUDE_DIRS ${mshiors_INCLUDE_DIR} )
    set( mshiors_LIBRARIES ${mshiors_LIBRARY} )
endif( mshiors_FOUND )
