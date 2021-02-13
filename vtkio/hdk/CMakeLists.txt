cmake_minimum_required( VERSION 3.6 )

project( Vtkio )

# Setup compile flags
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

if(NOT WIN32)
    set(CMAKE_CXX_FLAGS_DEBUG "-g")
    set(CMAKE_CXX_FLAGS_RELEASE "-O3")
endif()

# CMAKE_PREFIX_PATH must contain the path to the toolkit/cmake subdirectory of
# the Houdini installation. See the "Compiling with CMake" section of the HDK
# documentation for more details, which describes several options for
# specifying this path.
list( APPEND CMAKE_PREFIX_PATH "$ENV{HFS}/toolkit/cmake" )

# Add path for shared cmake modules like hdkrs
if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
    list( APPEND CMAKE_PREFIX_PATH "${CMAKE_SOURCE_DIR}/../../target/debug/cmake/")
else()
    list( APPEND CMAKE_PREFIX_PATH "${CMAKE_SOURCE_DIR}/../../target/release/cmake/")
endif()

# Set the directory of the library to be found by find_package
set( Vtkio_DIR ${CMAKE_SOURCE_DIR} )

# Locate Houdini's libraries and header files.
# Registers an imported library target named 'Houdini'.
find_package( Houdini REQUIRED )
find_package( hdkrs REQUIRED )
find_package( Vtkio REQUIRED )
find_package( Boost REQUIRED )

if(APPLE)
    find_library( MACOS_SECURITY Security )
endif()

set( library_name vtkio )

# Add a library and its source files.
add_library( ${library_name} SHARED
  src/GEO_Register.C
  src/GEO_VtkIO.C
  src/GEO_VtkIO.h
  src/GEO_VtuIO.C
  src/GEO_VtuIO.h
  src/GEO_VtpIO.C
  src/GEO_VtpIO.h
  src/add_mesh_visitor.h
)

# Link against the Houdini libraries, and add required include directories and
# compile definitions.
target_link_libraries( ${library_name} Houdini ${Vtkio_LIBRARIES} ${MACOS_SECURITY}
    ${Boost_LIBRARIES})

if(WIN32)
    # Add Rust specific Windows system dependencies.
    target_link_libraries( ${library_name} PRIVATE Ws2_32 userenv )
endif(WIN32)

# Include ${CMAKE_CURRENT_BINARY_DIR} for the generated header.
target_include_directories( ${library_name} PRIVATE
    ${CMAKE_CURRENT_BINARY_DIR} ${Vtkio_INCLUDE_DIRS} ${HDKRS_INCLUDE_DIRS}
    ${Boost_INCLUDE_DIRS}
)

# Sets several common target properties, such as the library's output directory.
houdini_configure_target( ${library_name} )
