cmake_minimum_required( VERSION 3.6 )

project( vtkiohdk )

if(NOT WIN32)
    set(CMAKE_CXX_FLAGS_DEBUG "-g")
    set(CMAKE_CXX_FLAGS_RELEASE "-O3")
endif()

# CMAKE_PREFIX_PATH must contain the path to the toolkit/cmake subdirectory of
# the Houdini installation. See the "Compiling with CMake" section of the HDK
# documentation for more details, which describes several options for
# specifying this path.
list( APPEND CMAKE_PREFIX_PATH "$ENV{HFS}/toolkit/cmake" )

# Locate Houdini's libraries and header files.
# Registers an imported library target named 'Houdini'.
find_package( Houdini REQUIRED )

# Find the rust build of the vtkio library.
list(APPEND CMAKE_PREFIX_PATH ${CMAKE_SOURCE_DIR})

find_package( vtkiors REQUIRED )

# Find the hdkrs include directories
file(READ "${CMAKE_BINARY_DIR}/rust/out_dir_hdkrs.txt" hdkrs_OUT_DIR)
list(APPEND CMAKE_PREFIX_PATH "${hdkrs_OUT_DIR}/cmake")

find_package( hdkrs REQUIRED )

if(APPLE)
    find_library( MACOS_SECURITY Security )
endif()

# The library name determines how we link it to the Rust lib in Cargo.toml
set( library_name vtkiohdk )

# Add a library and its source files.
add_library( ${library_name} SHARED
  src/GEO_Register.cpp
  src/GEO_VtkIO.cpp
  src/GEO_VtuIO.cpp
  src/GEO_VtpIO.cpp
)

set_property( TARGET ${library_name} PROPERTY CXX_STANDARD 17 )

# Link against the Houdini libraries, and add required include directories and
# compile definitions.
target_link_libraries( ${library_name} PUBLIC Houdini ${MACOS_SECURITY} PRIVATE ${vtkiors_LIBRARY})

if(WIN32)
    # Add Rust specific Windows system dependencies.
    target_link_libraries( ${library_name} PRIVATE Ws2_32 userenv bcrypt )
endif(WIN32)

target_include_directories( ${library_name} PRIVATE ${vtkiors_INCLUDE_DIRS} ${hdkrs_INCLUDE_DIRS} )

# Sets several common target properties, such as the library's output directory.
houdini_configure_target( ${library_name} )
