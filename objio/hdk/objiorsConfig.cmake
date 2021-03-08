if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
    set( objio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/debug" )
else()
    set( objio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/release" )
endif()

find_path( objiors_INCLUDE_DIR objio/src/lib.rs.h PATHS "${CMAKE_SOURCE_DIR}/../../target/cxxbridge" DOC "objiors include directory")
find_library( objiors_LIBRARY objiors PATHS ${objio_LIB_DIR} DOC "objiors library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args( objiors REQUIRED_VARS objiors_LIBRARY objiors_INCLUDE_DIR )

if( objiors_FOUND )
    set( objiors_INCLUDE_DIRS ${objiors_INCLUDE_DIR} )
    set( objiors_LIBRARIES ${objiors_LIBRARY} )
endif( objiors_FOUND )
