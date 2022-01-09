cmake_minimum_required( VERSION 3.6 )

project( cxxbridge-mshiohdk )

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
set( library_name cxxbridge-mshiohdk )

# Add a library and its source files.
add_library( ${library_name} STATIC "${CMAKE_BINARY_DIR}/../cxxbridge/sources/mshio/src/lib.rs.cc")

# Link against the Houdini libraries, and add required include directories and
# compile definitions.
target_link_libraries( ${library_name} Houdini )

# Sets several common target properties, such as the library's output directory.
houdini_configure_target( ${library_name} INSTDIR ${CMAKE_INSTALL_PREFIX} LIB_PREFIX lib)
