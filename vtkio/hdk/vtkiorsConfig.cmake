if("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
    set( vtkio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/debug" )
else()
    set( vtkio_LIB_DIR "${CMAKE_SOURCE_DIR}/../../target/release" )
endif()

find_path( vtkiors_INCLUDE_DIR vtkio/src/lib.rs.h PATHS "${CMAKE_SOURCE_DIR}/../../target/cxxbridge" DOC "vtkiors include directory")
find_library( vtkiors_LIBRARY vtkiors PATHS ${vtkio_LIB_DIR} DOC "vtkiors library directory")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args( vtkiors REQUIRED_VARS vtkiors_LIBRARY vtkiors_INCLUDE_DIR)

if( vtkiors_FOUND )
    set( vtkiors_INCLUDE_DIRS ${vtkiors_INCLUDE_DIR} )
    set( vtkiors_LIBRARIES ${vtkiors_LIBRARY} )
endif( vtkiors_FOUND )
