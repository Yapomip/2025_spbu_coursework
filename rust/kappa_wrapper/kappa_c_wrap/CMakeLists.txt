
cmake_minimum_required(VERSION 3.5)
project(kappa_c_wrap CXX)

include(FetchContent)
set(KAPPA_DATA_COPY ON)
set(KAPPA_DIST_DATA_DIR ${CMAKE_CURRENT_SOURCE_DIR})
FetchContent_Declare(
  kappa
  GIT_REPOSITORY https://github.com/Yapomip/kappa.git
  GIT_TAG        81d60ca21694a40bc80c42188df86810bc61bf02
)
FetchContent_MakeAvailable(kappa)
  
# add_subdirectory(../../../kappa ../../../kappa/build)
add_subdirectory(lib)

if (DEFINED KAPPA_C_WRAPPER_EXEMPLE AND KAPPA_C_WRAPPER_EXEMPLE)
  add_subdirectory(example)
endif()
