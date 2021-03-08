# This file finds all the needed header files needed for downstream crates.
find_path( cxxbridge_INCLUDE_DIR hdkrs/src/lib.rs.h PATHS "${hdkrs_DIR}/../cxxbridge/include" DOC "hdkrs cxxbridge include directory")
find_path( hdkrs_INCLUDE_DIR hdkrs/prelude.h PATHS "${hdkrs_DIR}/../include" DOC "hdkrs include directory")

# Setup find_package for downstream CMake projects.
include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(
    hdkrs 
    REQUIRED_VARS cxxbridge_INCLUDE_DIR hdkrs_INCLUDE_DIR
)

if( hdkrs_FOUND )
    set( hdkrs_INCLUDE_DIRS ${hdkrs_INCLUDE_DIR} ${cxxbridge_INCLUDE_DIR})
endif( hdkrs_FOUND )