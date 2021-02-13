if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
    set( HDKRS_DIR "${CMAKE_SOURCE_DIR}/../../target/debug" )
else()
    set( HDKRS_DIR "${CMAKE_SOURCE_DIR}/../../target/release" )
endif()

find_path( HDKRS_INCLUDE_DIR hdkrs/hdkrs.h PATHS ${HDKRS_DIR} DOC "hdkrs include directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
    hdkrs 
    REQUIRED_VARS HDKRS_INCLUDE_DIR
    )

if( HDKRS_FOUND )
    set( HDKRS_INCLUDE_DIRS ${HDKRS_INCLUDE_DIR} )
endif( HDKRS_FOUND )
