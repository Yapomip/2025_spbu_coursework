
cmake_minimum_required(VERSION 3.5...3.25)
project(kappa_c_wrap C CXX)

option(KAPPA_C_WRAP_EXAMPLE "Build example" OFF)

##########################################################################################################

# include(FetchContent)
# FetchContent_Declare(
#   kappa
#   GIT_REPOSITORY https://github.com/Yapomip/kappa.git
#   GIT_TAG        e0d55dc2b519888551b0591148f2c20f452c59b9
# )
# FetchContent_MakeAvailable(kappa)

##########################################################################################################

# add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../kappa ${CMAKE_CURRENT_SOURCE_DIR}/../kappa/build)

##########################################################################################################

if(NOT kappa++_DIR)
    set(kappa++_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../kappa/install/cmake)
endif()
find_package(kappa++ REQUIRED PATHS "${kappa++_DIR}")
message(STATUS "kappa++_DIR: ${kappa++_DIR}")

##########################################################################################################

add_subdirectory(src)

if(KAPPA_C_WRAP_EXAMPLE)
    add_subdirectory(example)
endif()

include(CMakePackageConfigHelpers)
configure_package_config_file(
   "Config.cmake.in"
   "${CMAKE_CURRENT_BINARY_DIR}/kappa_c_wrapConfig.cmake"
   INSTALL_DESTINATION "cmake"
)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/kappa_c_wrapConfig.cmake" DESTINATION "cmake")

