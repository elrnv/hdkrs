file(READ "${CMAKE_BINARY_DIR}/rust/out_dir_hdkrs.txt" HDKRS_DIR)

find_path(HDKRS_INCLUDE_DIR hdkrs/hdkrs.h PATHS ${HDKRS_DIR} DOC "hdkrs include directory")
find_library(HDKRS_LIBRARY hdkrs PATHS ${HDKRS_DIR} DOC "hdkrs library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
    hdkrs 
    REQUIRED_VARS HDKRS_INCLUDE_DIR
    )

if( HDKRS_FOUND )
    set( HDKRS_INCLUDE_DIRS ${HDKRS_INCLUDE_DIR} )
    set( HDKRS_LIBRARIES ${HDKRS_LIBRARY} )
endif( HDKRS_FOUND )
