cmake_minimum_required( VERSION 3.6 )

project( hdkrs )

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

if(APPLE)
    find_library( MACOS_SECURITY Security )
endif()

set( library_name hdkrs )

# Add a library and its source files.
add_library( ${library_name} STATIC
  src/mesh.cpp
  src/interrupt.cpp
  "${CMAKE_BINARY_DIR}/../cxxbridge/sources/hdkrs/src/lib.rs.cc"
)

set_property(TARGET ${library_name} PROPERTY CXX_STANDARD 17)

# Link against the Houdini libraries, and add required include directories and
# compile definitions.
target_link_libraries( ${library_name} Houdini ${MACOS_SECURITY})

if(WIN32)
    # Add Rust specific Windows system dependencies.
    target_link_libraries( ${library_name} Ws2_32 userenv )
endif(WIN32)

# Include ${CMAKE_CURRENT_BINARY_DIR} for the generated header.
target_include_directories( ${library_name} PRIVATE ${CMAKE_CURRENT_BINARY_DIR} "./include")

# Sets several common target properties, such as the library's output directory.
houdini_configure_target( ${library_name} INSTDIR ${CMAKE_INSTALL_PREFIX} LIB_PREFIX lib)
