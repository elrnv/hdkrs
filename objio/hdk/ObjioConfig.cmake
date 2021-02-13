if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
    set( Objio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/debug" )
else()
    set( Objio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/release" )
endif()

find_path( Objio_INCLUDE_DIR objio-hdk.h
    PATHS ${Objio_LIB_DIR}
    DOC "Objio include directory")
find_library( Objio_LIBRARY objiohdk PATHS ${Objio_LIB_DIR} DOC "Objio library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
    Objio
    REQUIRED_VARS Objio_LIBRARY Objio_INCLUDE_DIR
    )

if( Objio_FOUND )
    set( Objio_INCLUDE_DIRS ${Objio_INCLUDE_DIR} )
    set( Objio_LIBRARIES ${Objio_LIBRARY} )
endif( Objio_FOUND )


