
cmake_minimum_required(VERSION 3.5...3.25)
project(kappa_c_wrap C CXX)

include(FetchContent)

option(KAPPA_C_WRAP_EXAMPLE "Build example" OFF)

# FetchContent_Declare(
#   kappa
#   GIT_REPOSITORY https://github.com/Yapomip/kappa.git
#   GIT_TAG        81d60ca21694a40bc80c42188df86810bc61bf02
# )
# FetchContent_MakeAvailable(kappa)

# add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../kappa ${CMAKE_CURRENT_SOURCE_DIR}/../kappa/build)

if(NOT kappa++_DIR)
    set(kappa++_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../kappa/install/cmake)
endif()
find_package(kappa++ REQUIRED PATHS "${kappa++_DIR}")
message(STATUS "kappa++_DIR: ${kappa++_DIR}")

add_subdirectory(src)

if (KAPPA_C_WRAP_EXAMPLE)
    add_subdirectory(example)
endif()

if (DEFINED kappa++_DIR)
    include(CMakePackageConfigHelpers)
    configure_package_config_file(
        "Config.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/kappa_c_wrapConfig.cmake"
        INSTALL_DESTINATION "cmake"
    )
    install(FILES "${CMAKE_CURRENT_BINARY_DIR}/kappa_c_wrapConfig.cmake" DESTINATION "cmake")
endif()
