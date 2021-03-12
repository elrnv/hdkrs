cmake_minimum_required( VERSION 3.6 )

project( objiohdk )

# Setup compile flags
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

# Find the rust build of the objio library.
list(APPEND CMAKE_PREFIX_PATH ".")

find_package( objiors REQUIRED )

# Find the hdkrs include directories
file(READ "${CMAKE_BINARY_DIR}/rust/out_dir_hdkrs.txt" hdkrs_OUT_DIR)
list(APPEND CMAKE_PREFIX_PATH "${hdkrs_OUT_DIR}/cmake")

find_package( hdkrs REQUIRED )

if(APPLE)
    find_library( MACOS_SECURITY Security )
endif()

# The library name determines how we link this lib to the Rust lib in Cargo.toml
set( library_name objiohdk )

# Add a library and its source files.
# Notice that we generate a STATIC lib since this won't be directly installed into the dso folder,
# Instead we will install the Rust build output.
add_library( ${library_name} SHARED
  src/GEO_ObjIO.cpp
  src/GEO_ObjIO.h
)

set_property(TARGET ${library_name} PROPERTY CXX_STANDARD 17)

# Link against the Houdini libraries, and add required include directories and
# compile definitions.
target_link_libraries( ${library_name} Houdini ${MACOS_SECURITY} ${objiors_LIBRARY})

if(WIN32)
    # Add Rust specific Windows system dependencies.
    target_link_libraries( ${library_name} PRIVATE Ws2_32 userenv )
endif(WIN32)

target_include_directories( ${library_name} PRIVATE ${objiors_INCLUDE_DIRS} ${hdkrs_INCLUDE_DIRS} )

# Sets several common target properties, such as the library's output directory.
houdini_configure_target( ${library_name})