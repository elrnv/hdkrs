# This file finds all the needed header files needed for downstream crates.

# Find the output directory of the Rust build (this file is provided by cargo-hdk)
file(READ "${CMAKE_BINARY_DIR}/rust/out_dir_hdkrs.txt" HDKRS_DIR)
set(CXXBRIDGE_DIR "${HDKRS_DIR}/cxxbridge/include")

# Find the cxx bridge headers.
find_path(CXXBRIDGE_INCLUDE_DIR hdkrs/src/lib.rs.h PATHS ${CXXBRIDGE_DIR} DOC "cxxbridge include directory")
# Find the HDK specific headers.
find_path(HDKRS_INCLUDE_DIR hdkrs/prelude.h PATHS ${HDKRS_DIR} DOC "hdkrs include directory")

# Setup find_package for downstream CMake projects.
include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
    hdkrs 
    REQUIRED_VARS CXXBRIDGE_INCLUDE_DIR HDKRS_INCLUDE_DIR
)

if( HDKRS_FOUND )
    set( HDKRS_INCLUDE_DIRS ${HDKRS_INCLUDE_DIR} ${CXXBRIDGE_INCLUDE_DIR})
endif( HDKRS_FOUND )
