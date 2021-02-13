if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
    set( Vtkio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/debug" )
else()
    set( Vtkio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/release" )
endif()

find_path( Vtkio_INCLUDE_DIR vtkio-hdk.h
    PATHS ${Vtkio_LIB_DIR}
    DOC "Vtkio include directory")
find_library( Vtkio_LIBRARY vtkiohdk PATHS ${Vtkio_LIB_DIR} DOC "Vtkio library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
    Vtkio
    REQUIRED_VARS Vtkio_LIBRARY Vtkio_INCLUDE_DIR
    )

if( Vtkio_FOUND )
    set( Vtkio_INCLUDE_DIRS ${Vtkio_INCLUDE_DIR} )
    set( Vtkio_LIBRARIES ${Vtkio_LIBRARY} )
endif( Vtkio_FOUND )


